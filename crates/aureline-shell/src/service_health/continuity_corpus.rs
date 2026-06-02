//! Service-health continuity drill corpus.
//!
//! ## Why a corpus, not a single seeded packet
//!
//! The seeded aggregator at `crates/aureline-shell/src/service_health/seed.rs`
//! exercises the closed contract-state vocabulary in one mixed packet so
//! shell, About, CLI/headless inspect, diagnostics, and support exports
//! agree on a single render. That seed is enough to prove the cross-tool
//! boundary is wired up, but it is **not** enough to prove the beta-claim
//! invariants the service-health surface is graded against:
//!
//! - A single failed service cannot silently flip the whole product into
//!   broken or unavailable messaging when local work remains safe.
//! - Control-plane (release-channel, license broker, marketplace, status
//!   feed) and data-plane (workspace sync, remote runtime, language
//!   services) impairments stay distinguishable in product, docs, and
//!   support export — one outage class does not blur into the other.
//! - Mirror-only and offline-fallback paths show residual dependency and
//!   keep the user on local-safe guidance instead of generic unavailable
//!   language.
//! - Stale or never-checked probes never masquerade as current online
//!   truth: a cached `ready` card from before a restart lights the
//!   freshness warning so the chrome cannot paint the family green.
//! - Recovery after restart, reconnect, or offline transition is observable
//!   — the fresh probe replaces the cached "down" state instead of being
//!   added on top of it.
//!
//! The corpus mints one [`ContinuityScenario`] per named drill. Each
//! scenario builds a [`ServiceHealthAggregator`] from a deterministic set
//! of [`ServiceHealthProbeReading`]s and asserts the overall rollup the
//! surface MUST land on. The drills are pinned bit-for-bit on disk under
//! `fixtures/ops/m3/service_health_continuity/` so a regression in
//! contract-state wording, freshness, affected-workflow mapping, or the
//! local-continuity rollup fails the fixture-replay test instead of
//! shipping silently.

use super::aggregator::{
    AffectedWorkflowClass, BoundaryClass, LocalContinuityClass, ServiceContractStateClass,
    ServiceFamilyClass, ServiceHealthAggregator, ServiceHealthProbeReading,
};

/// Stable `as_of` instant the whole corpus is evaluated against. Pinned so
/// the on-disk fixtures stay deterministic.
pub const CORPUS_AS_OF: &str = "2026-05-19T12:30";

/// Stable aggregator-id prefix shared by every scenario.
pub const CORPUS_AGGREGATOR_ID_PREFIX: &str = "service_health_aggregator:m3.beta.continuity.";

/// Plane the drill exercises. Surfaces must keep control-plane and
/// data-plane impairment distinct in copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrillPlaneClass {
    /// Exactly one named service family is impaired. Used to prove the
    /// local-continuity invariant when the impaired family is hosted.
    SingleService,
    /// Multiple control-plane families (release channel, license broker,
    /// marketplace, status feed, telemetry) are impaired while data-plane
    /// editing is unaffected.
    ControlPlane,
    /// Data-plane families (workspace sync, remote runtime, language
    /// services) are impaired while control-plane truth keeps flowing.
    DataPlane,
    /// Primary path is unreachable; a mirror or cached fallback is
    /// serving. Used to prove the user reads "local-safe" guidance and
    /// not generic "unavailable" copy.
    MirrorFallback,
    /// Cards are serving cached or aged data; the chrome must not paint
    /// them as current online truth.
    StaleCache,
    /// Remote responded with a payload outside the agreed schema; results
    /// must be held until the contract clears.
    ContractMismatch,
    /// Policy / governance gate blocked the service; local work stays
    /// safe.
    PolicyBlock,
    /// Auth, license, or token state was lost; cascades through any
    /// family that needs the credential.
    AuthLoss,
    /// Previously impaired families are coming back. Used to prove the
    /// cached "down" state was discarded and replaced by fresh probes.
    Recovery,
}

impl DrillPlaneClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleService => "single_service",
            Self::ControlPlane => "control_plane",
            Self::DataPlane => "data_plane",
            Self::MirrorFallback => "mirror_fallback",
            Self::StaleCache => "stale_cache",
            Self::ContractMismatch => "contract_mismatch",
            Self::PolicyBlock => "policy_block",
            Self::AuthLoss => "auth_loss",
            Self::Recovery => "recovery",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleService => "Single-service outage",
            Self::ControlPlane => "Control-plane impairment",
            Self::DataPlane => "Data-plane impairment",
            Self::MirrorFallback => "Mirror / offline fallback",
            Self::StaleCache => "Stale cache",
            Self::ContractMismatch => "Contract mismatch",
            Self::PolicyBlock => "Policy block",
            Self::AuthLoss => "Auth / license loss",
            Self::Recovery => "Recovery after restart",
        }
    }
}

/// One drill. Surfaces under review MUST reproduce the same aggregator
/// projection bit-for-bit; the test in
/// `crates/aureline-shell/tests/service_health_continuity_fixtures.rs`
/// pins each scenario against the on-disk fixture under
/// `fixtures/ops/m3/service_health_continuity/`.
#[derive(Clone)]
pub struct ContinuityScenario {
    /// Stable identifier. Quoted in the matrix, the report, and the
    /// claim-qualification doc so a reviewer can cross-reference each
    /// drill.
    pub drill_id: &'static str,
    /// Stable human-readable label.
    pub drill_label: &'static str,
    /// Plane class the drill exercises.
    pub plane: DrillPlaneClass,
    /// One-sentence narrative the report and the matrix quote.
    pub narrative: &'static str,
    /// On-disk fixture filename (relative to
    /// `fixtures/ops/m3/service_health_continuity/`).
    pub fixture_filename: &'static str,
    /// Expected `overall_contract_state` the surface MUST land on.
    pub expected_overall_contract_state: ServiceContractStateClass,
    /// Expected `overall_local_continuity` the surface MUST land on.
    pub expected_overall_local_continuity: LocalContinuityClass,
    /// Expected `honesty_marker_present` value.
    pub expected_honesty_marker_present: bool,
    /// Closed list of service-family card ids the drill is graded against
    /// (the families the narrative names). The corpus may include
    /// supporting `ready` cards for context; this list pins only the
    /// families the drill is specifically proving behaviour on.
    pub focused_card_ids: &'static [&'static str],
    /// Reading-set factory. Stable across calls.
    pub readings_fn: fn() -> Vec<ServiceHealthProbeReading>,
}

impl ContinuityScenario {
    /// Build the aggregator the drill represents.
    pub fn aggregator(&self) -> ServiceHealthAggregator {
        let aggregator_id = format!("{}{}", CORPUS_AGGREGATOR_ID_PREFIX, self.drill_id);
        ServiceHealthAggregator::build(aggregator_id, CORPUS_AS_OF, (self.readings_fn)())
            .expect("continuity scenario must build")
    }
}

/// The complete corpus. Iteration order is the order surfaces and the
/// report read.
pub fn continuity_corpus() -> Vec<ContinuityScenario> {
    vec![
        ContinuityScenario {
            drill_id: "single_service_outage",
            drill_label: "Single-service outage — marketplace unreachable",
            plane: DrillPlaneClass::SingleService,
            narrative: "Only the hosted marketplace family is unreachable; the editor, language services, AI assist, and workspace sync stay current. Overall local continuity must remain local-safe so the chrome cannot imply the whole product is broken.",
            fixture_filename: "single_service_outage.json",
            expected_overall_contract_state: ServiceContractStateClass::Unavailable,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafe,
            expected_honesty_marker_present: true,
            focused_card_ids: &["card:marketplace"],
            readings_fn: readings_single_service_outage,
        },
        ContinuityScenario {
            drill_id: "control_plane_unavailable",
            drill_label: "Control-plane impairment — release, license, marketplace, status feed",
            plane: DrillPlaneClass::ControlPlane,
            narrative: "Release-channel response fails contract validation, the license broker is unreachable behind cached entitlement, marketplace is offline, and the status feed is degraded. Data-plane editing, sync, and AI assist stay current; the chrome must keep local continuity local-safe and surface contract-plane copy that is distinct from data-plane outage language.",
            fixture_filename: "control_plane_unavailable.json",
            expected_overall_contract_state: ServiceContractStateClass::ContractMismatch,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafe,
            expected_honesty_marker_present: true,
            focused_card_ids: &[
                "card:release_channel",
                "card:license_entitlement",
                "card:marketplace",
                "card:status_feed",
            ],
            readings_fn: readings_control_plane_unavailable,
        },
        ContinuityScenario {
            drill_id: "data_plane_unavailable",
            drill_label: "Data-plane impairment — workspace sync and remote runtime",
            plane: DrillPlaneClass::DataPlane,
            narrative: "Workspace sync is unreachable behind the remote-required boundary; the hosted remote runtime is offline. Control-plane truth (license, release channel, marketplace) keeps flowing. Local edits remain safe but external writes pause, so the rollup must land at local_safe_read_only — never local_safe.",
            fixture_filename: "data_plane_unavailable.json",
            expected_overall_contract_state: ServiceContractStateClass::Unavailable,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafeReadOnly,
            expected_honesty_marker_present: true,
            focused_card_ids: &["card:workspace_sync", "card:remote_runtime"],
            readings_fn: readings_data_plane_unavailable,
        },
        ContinuityScenario {
            drill_id: "mirror_only_fallback",
            drill_label: "Mirror / offline fallback — marketplace and docs serving cached mirror",
            plane: DrillPlaneClass::MirrorFallback,
            narrative: "Marketplace and docs primary paths are unreachable; the cached mirrors keep both families usable. The chrome must read local_only with mirror-class detail tokens, hold the cards honest about the fallback, and keep local continuity local-safe so the user is not steered into generic unavailable language.",
            fixture_filename: "mirror_only_fallback.json",
            expected_overall_contract_state: ServiceContractStateClass::LocalOnly,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafe,
            expected_honesty_marker_present: true,
            focused_card_ids: &["card:marketplace", "card:docs_knowledge"],
            readings_fn: readings_mirror_only_fallback,
        },
        ContinuityScenario {
            drill_id: "stale_cache",
            drill_label: "Stale cache — release channel, docs mirror, and status feed past review window",
            plane: DrillPlaneClass::StaleCache,
            narrative: "Probes for release channel, docs mirror, and the external status feed have not refreshed inside their review windows. Cached responses are still being served; the cards must surface stale + very_stale age buckets and the honesty marker — they MUST NOT render as current ready truth.",
            fixture_filename: "stale_cache.json",
            expected_overall_contract_state: ServiceContractStateClass::Stale,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafe,
            expected_honesty_marker_present: true,
            focused_card_ids: &[
                "card:release_channel",
                "card:docs_knowledge",
                "card:status_feed",
            ],
            readings_fn: readings_stale_cache,
        },
        ContinuityScenario {
            drill_id: "contract_mismatch",
            drill_label: "Contract mismatch — release channel returned an off-schema manifest",
            plane: DrillPlaneClass::ContractMismatch,
            narrative: "The release channel responded with a payload that did not match the agreed claim-manifest schema. Results are held until the contract clears. The chrome must NOT collapse contract_mismatch into generic degraded copy.",
            fixture_filename: "contract_mismatch.json",
            expected_overall_contract_state: ServiceContractStateClass::ContractMismatch,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafe,
            expected_honesty_marker_present: true,
            focused_card_ids: &["card:release_channel"],
            readings_fn: readings_contract_mismatch,
        },
        ContinuityScenario {
            drill_id: "policy_block",
            drill_label: "Policy block — telemetry and AI assist gated by workspace policy",
            plane: DrillPlaneClass::PolicyBlock,
            narrative: "Workspace policy disabled telemetry upload and AI assist. Local edits, sync, and language services keep working. The chrome must surface policy_blocked — distinct from unavailable — and explain that local work continues to be safe.",
            fixture_filename: "policy_block.json",
            expected_overall_contract_state: ServiceContractStateClass::PolicyBlocked,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafe,
            expected_honesty_marker_present: true,
            focused_card_ids: &["card:telemetry", "card:ai_assist"],
            readings_fn: readings_policy_block,
        },
        ContinuityScenario {
            drill_id: "auth_loss",
            drill_label: "Auth / license loss — broker unreachable cascades into sync push",
            plane: DrillPlaneClass::AuthLoss,
            narrative: "The license broker is unreachable; cached entitlements remain honoured for local work. Sync needs a fresh token to push, so it falls back to local_only behind the remote-required boundary. Overall local continuity must land at local_safe_read_only.",
            fixture_filename: "auth_loss.json",
            expected_overall_contract_state: ServiceContractStateClass::Unavailable,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafeReadOnly,
            expected_honesty_marker_present: true,
            focused_card_ids: &["card:license_entitlement", "card:workspace_sync"],
            readings_fn: readings_auth_loss,
        },
        ContinuityScenario {
            drill_id: "recovery_after_restart",
            drill_label: "Recovery after restart — previously down services replaced by fresh probes",
            plane: DrillPlaneClass::Recovery,
            narrative: "After restart, workspace sync, marketplace, and the license broker are reachable again and probes returned current. The AI provider has not yet replied for this restart cycle; the chrome must paint AI as never_checked rather than reusing the pre-restart 'ready' cache.",
            fixture_filename: "recovery_after_restart.json",
            expected_overall_contract_state: ServiceContractStateClass::Ready,
            expected_overall_local_continuity: LocalContinuityClass::LocalSafe,
            // honesty marker stays true because the AI card has never_checked
            // freshness — recovery is honest about what hasn't been re-probed.
            expected_honesty_marker_present: true,
            focused_card_ids: &[
                "card:workspace_sync",
                "card:marketplace",
                "card:license_entitlement",
                "card:ai_assist",
            ],
            readings_fn: readings_recovery_after_restart,
        },
    ]
}

// ---------------------------------------------------------------------------
// Reading-set builders. Pure functions; calling them twice yields identical
// vectors so the aggregator output is deterministic.
// ---------------------------------------------------------------------------

fn ready_language_services() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:language_services".to_owned(),
        service_family: ServiceFamilyClass::LanguageServices,
        boundary_class: BoundaryClass::LocalOnly,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:28".to_owned()),
        state_explanation: "Language services are responding within their normal latency budget."
            .to_owned(),
        diagnostics_action: "shell.command:diagnostics.language_services".to_owned(),
        detail_tokens: vec!["framework_lsp".to_owned(), "local_indexer".to_owned()],
    }
}

fn ready_ai_assist() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:ai_assist".to_owned(),
        service_family: ServiceFamilyClass::AiAssist,
        boundary_class: BoundaryClass::VendorProvider,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:29".to_owned()),
        state_explanation: "AI provider is responding within its expected latency.".to_owned(),
        diagnostics_action: "shell.command:diagnostics.ai_assist".to_owned(),
        detail_tokens: vec!["provider_class:vendor_chat".to_owned()],
    }
}

fn ready_workspace_sync() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:workspace_sync".to_owned(),
        service_family: ServiceFamilyClass::Sync,
        boundary_class: BoundaryClass::LocalWithRemoteRequired,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:27".to_owned()),
        state_explanation: "Workspace sync is reachable and pushes are current.".to_owned(),
        diagnostics_action: "shell.command:diagnostics.workspace_sync".to_owned(),
        detail_tokens: vec!["sync_class:workspace_sync".to_owned()],
    }
}

fn ready_license_entitlement() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:license_entitlement".to_owned(),
        service_family: ServiceFamilyClass::LicenseEntitlement,
        boundary_class: BoundaryClass::LocalWithRemoteOptional,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:26".to_owned()),
        state_explanation: "License broker is current; local entitlements are honoured.".to_owned(),
        diagnostics_action: "shell.command:diagnostics.license".to_owned(),
        detail_tokens: vec!["broker_class:local_broker".to_owned()],
    }
}

fn ready_release_channel() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:release_channel".to_owned(),
        service_family: ServiceFamilyClass::ReleaseChannel,
        boundary_class: BoundaryClass::LocalWithRemoteOptional,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:25".to_owned()),
        state_explanation: "Release-channel manifest matches the agreed schema.".to_owned(),
        diagnostics_action: "shell.command:diagnostics.release_channel".to_owned(),
        detail_tokens: vec!["channel_class:beta".to_owned()],
    }
}

fn ready_marketplace() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:marketplace".to_owned(),
        service_family: ServiceFamilyClass::Marketplace,
        boundary_class: BoundaryClass::Hosted,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:24".to_owned()),
        state_explanation: "Marketplace fetch is reachable and the catalogue is current."
            .to_owned(),
        diagnostics_action: "shell.command:diagnostics.marketplace".to_owned(),
        detail_tokens: vec!["mirror_class:marketplace_mirror".to_owned()],
    }
}

fn ready_telemetry() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:telemetry".to_owned(),
        service_family: ServiceFamilyClass::Telemetry,
        boundary_class: BoundaryClass::Hosted,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:26".to_owned()),
        state_explanation: "Telemetry upload is current and crash capture is flushing.".to_owned(),
        diagnostics_action: "shell.command:diagnostics.telemetry".to_owned(),
        detail_tokens: vec!["policy_class:workspace_policy".to_owned()],
    }
}

fn ready_remote_runtime() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:remote_runtime".to_owned(),
        service_family: ServiceFamilyClass::RemoteRuntime,
        boundary_class: BoundaryClass::Hosted,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:27".to_owned()),
        state_explanation: "Remote runtime is reachable; remote shell is responding.".to_owned(),
        diagnostics_action: "shell.command:diagnostics.remote_runtime".to_owned(),
        detail_tokens: vec!["runtime_class:remote_container".to_owned()],
    }
}

fn ready_docs_knowledge() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:docs_knowledge".to_owned(),
        service_family: ServiceFamilyClass::DocsKnowledge,
        boundary_class: BoundaryClass::LocalWithRemoteOptional,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:25".to_owned()),
        state_explanation: "Docs mirror refreshed inside its review window.".to_owned(),
        diagnostics_action: "shell.command:diagnostics.docs_mirror".to_owned(),
        detail_tokens: vec!["mirror_class:docs_mirror".to_owned()],
    }
}

fn ready_status_feed() -> ServiceHealthProbeReading {
    ServiceHealthProbeReading {
        card_id: "card:status_feed".to_owned(),
        service_family: ServiceFamilyClass::StatusFeed,
        boundary_class: BoundaryClass::Hosted,
        contract_state: ServiceContractStateClass::Ready,
        local_continuity: LocalContinuityClass::LocalSafe,
        affected_workflows: vec![],
        last_checked: Some("2026-05-19T12:28".to_owned()),
        state_explanation: "Status feed is reachable and the incident timeline is current."
            .to_owned(),
        diagnostics_action: "shell.command:diagnostics.status_feed".to_owned(),
        detail_tokens: vec!["feed_class:vendor_status".to_owned()],
    }
}

fn readings_single_service_outage() -> Vec<ServiceHealthProbeReading> {
    let mut marketplace = ready_marketplace();
    marketplace.contract_state = ServiceContractStateClass::Unavailable;
    marketplace.last_checked = Some("2026-05-19T12:18".to_owned());
    marketplace.affected_workflows = vec![
        AffectedWorkflowClass::MarketplaceBrowse,
        AffectedWorkflowClass::ExtensionInstall,
    ];
    marketplace.state_explanation =
        "Marketplace fetch is unreachable; installed extensions remain usable.".to_owned();

    vec![
        ready_language_services(),
        ready_ai_assist(),
        ready_workspace_sync(),
        marketplace,
    ]
}

fn readings_control_plane_unavailable() -> Vec<ServiceHealthProbeReading> {
    let mut release_channel = ready_release_channel();
    release_channel.contract_state = ServiceContractStateClass::ContractMismatch;
    release_channel.last_checked = Some("2026-05-19T12:10".to_owned());
    release_channel.state_explanation =
        "Release-channel response did not match the agreed manifest schema; results are held until the contract clears.".to_owned();
    release_channel.detail_tokens = vec![
        "channel_class:beta".to_owned(),
        "schema_skew:claim_manifest".to_owned(),
    ];

    let mut license = ready_license_entitlement();
    license.contract_state = ServiceContractStateClass::Unavailable;
    license.last_checked = Some("2026-05-19T12:12".to_owned());
    license.affected_workflows = vec![AffectedWorkflowClass::LicenseRefresh];
    license.state_explanation =
        "License broker is unreachable; cached entitlements remain honoured for local work."
            .to_owned();

    let mut marketplace = ready_marketplace();
    marketplace.contract_state = ServiceContractStateClass::Unavailable;
    marketplace.last_checked = Some("2026-05-19T12:18".to_owned());
    marketplace.affected_workflows = vec![
        AffectedWorkflowClass::MarketplaceBrowse,
        AffectedWorkflowClass::ExtensionInstall,
    ];
    marketplace.state_explanation =
        "Marketplace fetch is unreachable; installed extensions remain usable.".to_owned();

    let mut status_feed = ready_status_feed();
    status_feed.contract_state = ServiceContractStateClass::Degraded;
    status_feed.last_checked = Some("2026-05-19T12:14".to_owned());
    status_feed.state_explanation =
        "Status feed is slow; incident updates are arriving with retries.".to_owned();

    vec![
        ready_language_services(),
        ready_ai_assist(),
        ready_workspace_sync(),
        license,
        marketplace,
        release_channel,
        status_feed,
    ]
}

fn readings_data_plane_unavailable() -> Vec<ServiceHealthProbeReading> {
    let mut sync = ready_workspace_sync();
    sync.contract_state = ServiceContractStateClass::LocalOnly;
    sync.local_continuity = LocalContinuityClass::LocalSafeReadOnly;
    sync.last_checked = Some("2026-05-19T12:20".to_owned());
    sync.affected_workflows = vec![AffectedWorkflowClass::WorkspaceSync];
    sync.state_explanation =
        "Workspace sync is unreachable; local edits keep working, pushes pause until reconnect."
            .to_owned();
    sync.detail_tokens = vec!["last_synced_offline".to_owned()];

    let mut remote_runtime = ready_remote_runtime();
    remote_runtime.contract_state = ServiceContractStateClass::Unavailable;
    remote_runtime.last_checked = Some("2026-05-19T12:16".to_owned());
    remote_runtime.affected_workflows = vec![AffectedWorkflowClass::RemoteShell];
    remote_runtime.state_explanation =
        "Remote runtime is unreachable; local sandboxes continue to run.".to_owned();

    vec![
        ready_language_services(),
        ready_ai_assist(),
        sync,
        ready_license_entitlement(),
        remote_runtime,
        ready_release_channel(),
    ]
}

fn readings_mirror_only_fallback() -> Vec<ServiceHealthProbeReading> {
    let mut marketplace = ready_marketplace();
    marketplace.contract_state = ServiceContractStateClass::LocalOnly;
    marketplace.last_checked = Some("2026-05-19T12:18".to_owned());
    marketplace.affected_workflows = vec![AffectedWorkflowClass::MarketplaceBrowse];
    marketplace.state_explanation =
        "Marketplace primary fetch unreachable; the cached mirror is serving browse results."
            .to_owned();
    marketplace.detail_tokens = vec![
        "fallback_mode:mirror_only".to_owned(),
        "mirror_class:marketplace_mirror".to_owned(),
    ];

    let mut docs = ready_docs_knowledge();
    docs.contract_state = ServiceContractStateClass::LocalOnly;
    docs.last_checked = Some("2026-05-19T12:21".to_owned());
    docs.affected_workflows = vec![AffectedWorkflowClass::DocsBrowseRemote];
    docs.state_explanation =
        "Docs primary unreachable; the local docs mirror keeps reference lookups available."
            .to_owned();
    docs.detail_tokens = vec![
        "fallback_mode:mirror_only".to_owned(),
        "mirror_class:docs_mirror".to_owned(),
    ];

    vec![
        ready_language_services(),
        ready_ai_assist(),
        ready_workspace_sync(),
        marketplace,
        ready_release_channel(),
        docs,
    ]
}

fn readings_stale_cache() -> Vec<ServiceHealthProbeReading> {
    let mut release_channel = ready_release_channel();
    release_channel.contract_state = ServiceContractStateClass::Stale;
    release_channel.last_checked = Some("2026-05-17T08:00".to_owned());
    release_channel.state_explanation =
        "Release-channel manifest has not refreshed inside its review window; cached manifest is being served.".to_owned();

    let mut docs = ready_docs_knowledge();
    docs.contract_state = ServiceContractStateClass::Stale;
    docs.last_checked = Some("2026-05-18T08:00".to_owned());
    docs.affected_workflows = vec![AffectedWorkflowClass::DocsBrowseRemote];
    docs.state_explanation =
        "Docs mirror has not refreshed inside its review window; local docs remain available."
            .to_owned();

    let mut status_feed = ready_status_feed();
    status_feed.contract_state = ServiceContractStateClass::Stale;
    status_feed.last_checked = Some("2026-05-18T06:00".to_owned());
    status_feed.state_explanation =
        "Status feed has not refreshed inside its review window; cached incidents are being served.".to_owned();

    vec![
        ready_language_services(),
        ready_ai_assist(),
        ready_workspace_sync(),
        release_channel,
        docs,
        status_feed,
    ]
}

fn readings_contract_mismatch() -> Vec<ServiceHealthProbeReading> {
    let mut release_channel = ready_release_channel();
    release_channel.contract_state = ServiceContractStateClass::ContractMismatch;
    release_channel.last_checked = Some("2026-05-19T12:10".to_owned());
    release_channel.state_explanation =
        "Release-channel response did not match the agreed manifest schema; results are held until the contract clears.".to_owned();
    release_channel.detail_tokens = vec![
        "channel_class:beta".to_owned(),
        "schema_skew:claim_manifest".to_owned(),
    ];

    vec![
        ready_language_services(),
        ready_ai_assist(),
        ready_workspace_sync(),
        ready_license_entitlement(),
        release_channel,
    ]
}

fn readings_policy_block() -> Vec<ServiceHealthProbeReading> {
    let mut telemetry = ready_telemetry();
    telemetry.contract_state = ServiceContractStateClass::PolicyBlocked;
    telemetry.last_checked = Some("2026-05-19T12:15".to_owned());
    telemetry.affected_workflows = vec![AffectedWorkflowClass::TelemetryUpload];
    telemetry.state_explanation =
        "Telemetry upload is paused by workspace policy; local crash capture continues to write to disk.".to_owned();

    let mut ai_assist = ready_ai_assist();
    ai_assist.contract_state = ServiceContractStateClass::PolicyBlocked;
    ai_assist.last_checked = Some("2026-05-19T12:17".to_owned());
    ai_assist.affected_workflows = vec![
        AffectedWorkflowClass::AiCompletion,
        AffectedWorkflowClass::AiChat,
        AffectedWorkflowClass::AiInlineEdit,
    ];
    ai_assist.state_explanation =
        "AI assist is disabled by workspace policy; local editing and language services keep working.".to_owned();
    ai_assist.detail_tokens = vec![
        "policy_class:workspace_policy".to_owned(),
        "provider_class:vendor_chat".to_owned(),
    ];

    vec![
        ready_language_services(),
        ai_assist,
        ready_workspace_sync(),
        ready_license_entitlement(),
        telemetry,
        ready_release_channel(),
    ]
}

fn readings_auth_loss() -> Vec<ServiceHealthProbeReading> {
    let mut license = ready_license_entitlement();
    license.contract_state = ServiceContractStateClass::Unavailable;
    license.last_checked = Some("2026-05-19T12:18".to_owned());
    license.affected_workflows = vec![AffectedWorkflowClass::LicenseRefresh];
    license.state_explanation =
        "License broker is unreachable; cached entitlements remain honoured for local work."
            .to_owned();

    let mut sync = ready_workspace_sync();
    sync.contract_state = ServiceContractStateClass::LocalOnly;
    sync.local_continuity = LocalContinuityClass::LocalSafeReadOnly;
    sync.last_checked = Some("2026-05-19T12:19".to_owned());
    sync.affected_workflows = vec![AffectedWorkflowClass::WorkspaceSync];
    sync.state_explanation =
        "Workspace sync token expired and cannot be refreshed; local edits keep working, pushes pause until the broker recovers.".to_owned();
    sync.detail_tokens = vec![
        "auth_class:broker_token".to_owned(),
        "last_synced_offline".to_owned(),
    ];

    vec![
        ready_language_services(),
        ready_ai_assist(),
        sync,
        license,
        ready_release_channel(),
    ]
}

fn readings_recovery_after_restart() -> Vec<ServiceHealthProbeReading> {
    let mut sync = ready_workspace_sync();
    sync.last_checked = Some("2026-05-19T12:28".to_owned());
    sync.state_explanation =
        "Workspace sync reconnected after restart; pushes are current again.".to_owned();

    let mut marketplace = ready_marketplace();
    marketplace.last_checked = Some("2026-05-19T12:28".to_owned());
    marketplace.state_explanation =
        "Marketplace fetch reconnected after restart; the catalogue is current again.".to_owned();

    let mut license = ready_license_entitlement();
    license.last_checked = Some("2026-05-19T12:29".to_owned());
    license.state_explanation =
        "License broker reconnected after restart; entitlements are current again.".to_owned();

    let mut ai_assist = ready_ai_assist();
    ai_assist.last_checked = None;
    ai_assist.state_explanation =
        "AI provider has not yet been probed since restart; the chrome must not reuse the pre-restart status.".to_owned();
    ai_assist.detail_tokens = vec![
        "provider_class:vendor_chat".to_owned(),
        "post_restart_pending_probe".to_owned(),
    ];

    vec![
        ready_language_services(),
        ai_assist,
        sync,
        license,
        marketplace,
        ready_release_channel(),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::aggregator::LastCheckedAgeClass;
    use super::*;

    #[test]
    fn corpus_is_deterministic() {
        let a: Vec<_> = continuity_corpus()
            .into_iter()
            .map(|s| s.aggregator())
            .collect();
        let b: Vec<_> = continuity_corpus()
            .into_iter()
            .map(|s| s.aggregator())
            .collect();
        assert_eq!(a, b);
    }

    #[test]
    fn every_scenario_meets_its_pinned_expectations() {
        for scenario in continuity_corpus() {
            let agg = scenario.aggregator();
            assert_eq!(
                agg.overall_contract_state, scenario.expected_overall_contract_state,
                "{} overall_contract_state drift",
                scenario.drill_id,
            );
            assert_eq!(
                agg.overall_local_continuity, scenario.expected_overall_local_continuity,
                "{} overall_local_continuity drift",
                scenario.drill_id,
            );
            assert_eq!(
                agg.honesty_marker_present, scenario.expected_honesty_marker_present,
                "{} honesty_marker_present drift",
                scenario.drill_id,
            );
        }
    }

    #[test]
    fn focused_card_ids_resolve_in_each_scenario() {
        for scenario in continuity_corpus() {
            let agg = scenario.aggregator();
            for card_id in scenario.focused_card_ids {
                assert!(
                    agg.cards.iter().any(|c| c.card_id == *card_id),
                    "{} missing focused card {}",
                    scenario.drill_id,
                    card_id,
                );
            }
        }
    }

    #[test]
    fn hosted_or_vendor_outage_does_not_drag_overall_continuity_below_local_safe() {
        let scenario = continuity_corpus()
            .into_iter()
            .find(|s| s.drill_id == "single_service_outage")
            .expect("single_service_outage scenario must exist");
        let agg = scenario.aggregator();
        assert_eq!(
            agg.overall_local_continuity,
            LocalContinuityClass::LocalSafe,
            "hosted-only outage must not downgrade overall local continuity",
        );
    }

    #[test]
    fn data_plane_outage_downgrades_overall_continuity_to_at_most_read_only() {
        let scenario = continuity_corpus()
            .into_iter()
            .find(|s| s.drill_id == "data_plane_unavailable")
            .expect("data_plane_unavailable scenario must exist");
        let agg = scenario.aggregator();
        assert!(
            agg.overall_local_continuity <= LocalContinuityClass::LocalSafeReadOnly,
            "data-plane outage must downgrade overall continuity to at most local_safe_read_only",
        );
    }

    #[test]
    fn stale_cards_never_paint_as_fresh_or_ready() {
        let scenario = continuity_corpus()
            .into_iter()
            .find(|s| s.drill_id == "stale_cache")
            .expect("stale_cache scenario must exist");
        let agg = scenario.aggregator();
        for card in &agg.cards {
            if card.contract_state == ServiceContractStateClass::Stale {
                assert!(
                    matches!(
                        card.last_checked_age,
                        LastCheckedAgeClass::Stale
                            | LastCheckedAgeClass::VeryStale
                            | LastCheckedAgeClass::NeverChecked
                    ),
                    "stale card {} has age bucket {:?}; aged probes must not paint as fresh",
                    card.card_id,
                    card.last_checked_age,
                );
                assert!(
                    card.honesty_marker_present,
                    "stale card {} did not light the honesty marker",
                    card.card_id,
                );
            }
        }
    }

    #[test]
    fn recovery_drill_clears_cached_down_state_but_remains_honest_about_pending_probe() {
        let scenario = continuity_corpus()
            .into_iter()
            .find(|s| s.drill_id == "recovery_after_restart")
            .expect("recovery_after_restart scenario must exist");
        let agg = scenario.aggregator();
        let sync = agg
            .cards
            .iter()
            .find(|c| c.card_id == "card:workspace_sync")
            .expect("recovery scenario must include sync card");
        assert_eq!(
            sync.contract_state,
            ServiceContractStateClass::Ready,
            "post-restart sync must read ready, not the pre-restart cached state",
        );
        assert_eq!(sync.last_checked_age, LastCheckedAgeClass::Fresh);
        let ai = agg
            .cards
            .iter()
            .find(|c| c.card_id == "card:ai_assist")
            .expect("recovery scenario must include ai_assist card");
        assert_eq!(ai.last_checked_age, LastCheckedAgeClass::NeverChecked);
        assert!(
            ai.honesty_marker_present,
            "never-checked card must light the honesty marker so the chrome cannot reuse the pre-restart status",
        );
    }
}
