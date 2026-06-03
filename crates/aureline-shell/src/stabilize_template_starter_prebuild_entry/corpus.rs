//! Cross-surface template, starter, and prebuild entry disclosure drill corpus.
//!
//! The model proves the bypass-parity, source-honesty, runtime-consistency,
//! freshness-truth, failure-transparency, and export-safety invariants in
//! isolation. This corpus mints one [`TemplateStarterPrebuildEntryScenario`] per
//! named drill and pins each rendered [`TemplateStarterPrebuildEntryRecord`]
//! bit-for-bit on disk under
//! `fixtures/ux/m4/stabilize-template-starter-prebuild-entry/`, so a regression
//! in any invariant fails the fixture-replay test instead of shipping silently.

use super::model::{
    AcceleratorIdentity, BypassPath, BypassPathClass, CleanupRollback, CredentialProvisioningClass,
    EntryKind, ExtensionInstallClass, FailureOutcomeClass, FailureSummary, FailureSummaryItem,
    FreshnessClass, FreshnessReview, HostBoundaryClass, ManagedServiceClass, NetworkEgressClass,
    RemoteProvisioningClass, ResultingMode, RuntimeReview, RuntimeScopeClass, SetupActionClass,
    SetupReview, SideEffectEnvelope, SourceClass, SourceReview, SupportClass,
    SupportExportMetadata, SupportReview, TemplateStarterPrebuildEntryInput,
    TemplateStarterPrebuildEntryRecord, TrustAuthBoundaries, TrustPostureClass,
};

/// Stable `as_of` instant the whole corpus is evaluated against.
pub const CORPUS_AS_OF: &str = "2026-06-03T12:00:00Z";

/// Stable record-id prefix shared by every scenario.
pub const CORPUS_RECORD_ID_PREFIX: &str = "template_starter_prebuild_entry:m4.stable.corpus.";

/// One drill scenario.
#[derive(Clone)]
pub struct TemplateStarterPrebuildEntryScenario {
    /// Stable identifier.
    pub scenario_id: &'static str,
    /// Stable human-readable label.
    pub scenario_label: &'static str,
    /// One-sentence narrative.
    pub narrative: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Expected entry kind.
    pub expected_entry_kind: EntryKind,
    /// Expected resulting mode.
    pub expected_resulting_mode: ResultingMode,
    /// Expected honesty marker.
    pub expected_honesty_marker_present: bool,
    /// Expected bypass path count.
    pub expected_bypass_path_count: usize,
    input: TemplateStarterPrebuildEntryInput,
}

impl TemplateStarterPrebuildEntryScenario {
    /// Build the rendered record for this scenario.
    pub fn record(&self) -> TemplateStarterPrebuildEntryRecord {
        TemplateStarterPrebuildEntryRecord::build(self.input.clone())
            .expect("corpus scenario must build")
    }
}

// ---------------------------------------------------------------------------
// Compact constructors
// ---------------------------------------------------------------------------

fn identity(
    id: &str,
    label: &str,
    summary: &str,
    version: &str,
    manifest: &str,
    kind: EntryKind,
) -> AcceleratorIdentity {
    AcceleratorIdentity {
        accelerator_id: id.to_string(),
        display_label: label.to_string(),
        summary: summary.to_string(),
        accelerator_version: version.to_string(),
        bound_manifest_ref: manifest.to_string(),
        entry_kind: kind,
    }
}

fn source(
    class: SourceClass,
    dist: &str,
    sig: &str,
    publisher: &str,
    trust: &str,
    notes: Vec<&str>,
) -> SourceReview {
    SourceReview {
        source_class: class,
        source_distribution_class: dist.to_string(),
        signature_state: sig.to_string(),
        publisher_label: publisher.to_string(),
        trust_root_ref: trust.to_string(),
        trust_notes: notes.into_iter().map(String::from).collect(),
    }
}

fn support(class: SupportClass, lifecycle: &str) -> SupportReview {
    SupportReview {
        support_class: class,
        lifecycle_class: lifecycle.to_string(),
    }
}

fn runtime(
    scope: RuntimeScopeClass,
    host: HostBoundaryClass,
    ecosystems: Vec<&str>,
    platforms: Vec<&str>,
) -> RuntimeReview {
    RuntimeReview {
        runtime_scope_class: scope,
        host_boundary_class: host,
        supported_ecosystems: ecosystems.into_iter().map(String::from).collect(),
        supported_platforms: platforms.into_iter().map(String::from).collect(),
    }
}

fn freshness(
    class: FreshnessClass,
    age: Option<u64>,
    max: Option<u64>,
    producer: &str,
    signer: &str,
) -> FreshnessReview {
    FreshnessReview {
        freshness_class: class,
        age_seconds: age,
        max_age_seconds: max,
        producer_class: producer.to_string(),
        signer_posture: signer.to_string(),
    }
}

fn setup(
    actions: Vec<SetupActionClass>,
    duration: &str,
    connectivity: bool,
    expectation: &str,
) -> SetupReview {
    SetupReview {
        expected_actions: actions,
        estimated_duration_label: duration.to_string(),
        connectivity_required: connectivity,
        connectivity_expectation_label: expectation.to_string(),
    }
}

fn side_effects(
    egress: NetworkEgressClass,
    ext: ExtensionInstallClass,
    remote: RemoteProvisioningClass,
    managed: ManagedServiceClass,
    cred: CredentialProvisioningClass,
    hooks: u32,
    tasks: u32,
    notes: Vec<&str>,
) -> SideEffectEnvelope {
    SideEffectEnvelope {
        required_network_egress_class: egress,
        required_extension_install_class: ext,
        required_remote_provisioning_class: remote,
        required_managed_service_class: managed,
        required_credential_provisioning_class: cred,
        declared_hook_count: hooks,
        declared_setup_task_count: tasks,
        side_effect_notes: notes.into_iter().map(String::from).collect(),
    }
}

fn bypass(class: BypassPathClass, shortcut: Option<&str>) -> BypassPath {
    BypassPath {
        path_class: class,
        route_label: class.label().to_string(),
        bypass_continuity_class: "equal_weight_with_apply".to_string(),
        keyboard_shortcut_hint: shortcut.map(String::from),
    }
}

fn trust_auth(
    trust: TrustPostureClass,
    auth: bool,
    registry: bool,
    managed: &str,
    download: &str,
    significant: &str,
) -> TrustAuthBoundaries {
    TrustAuthBoundaries {
        trust_posture_class: trust,
        auth_required: auth,
        registry_mirror_required: registry,
        managed_service_boundary: managed.to_string(),
        download_provisioning_boundary: download.to_string(),
        significant_download_label: significant.to_string(),
    }
}

fn cleanup(
    cleanup: bool,
    rollback: bool,
    cleanup_sum: &str,
    rollback_sum: &str,
) -> CleanupRollback {
    CleanupRollback {
        cleanup_path_available: cleanup,
        rollback_path_available: rollback,
        cleanup_summary: cleanup_sum.to_string(),
        rollback_summary: rollback_sum.to_string(),
    }
}

fn support_meta() -> SupportExportMetadata {
    SupportExportMetadata {
        exportable: true,
        redaction_class: "metadata_safe_default".to_string(),
        versioned_identity_included: true,
        side_effect_envelope_included: true,
        raw_secret_export_allowed: false,
        raw_command_export_allowed: false,
        raw_url_export_allowed: false,
    }
}

fn failure_item(label: &str, outcome: FailureOutcomeClass, detail: &str) -> FailureSummaryItem {
    FailureSummaryItem {
        item_label: label.to_string(),
        outcome,
        detail: detail.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Scenario inputs
// ---------------------------------------------------------------------------

fn template_first_party_create_project() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "template_first_party_create_project",
        scenario_label: "First-party template — create project",
        narrative: "A first-party workspace template that creates a Rust project with no network egress and an equal-weight bypass to create an empty workspace.",
        fixture_filename: "template_first_party_create_project.json",
        expected_entry_kind: EntryKind::Template,
        expected_resulting_mode: ResultingMode::CreateProject,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}template_first_party_create_project"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "aureline.template.rust.default",
                "Rust project",
                "Creates a new Rust project with Cargo workspace, test scaffold, and CI configuration.",
                "2026.06.01",
                "aureline://template/rust_default",
                EntryKind::Template,
            ),
            source_review: source(
                SourceClass::FirstParty,
                "bundled",
                "signed_verified",
                "Aureline",
                "aureline://trust/first_party",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalOnly,
                HostBoundaryClass::HostLocalDeviceOnly,
                vec!["rust"],
                vec!["darwin-aarch64", "linux-x86_64", "windows-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::FreshUnderWindow, Some(0), Some(86400), "first_party_template_pipeline", "signed_verified"),
            setup_review: setup(
                vec![SetupActionClass::TrustWorkspace],
                "Under one minute",
                false,
                "No network required",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::NoNetworkEgressRequired,
                ExtensionInstallClass::NoExtensionInstallRequired,
                RemoteProvisioningClass::NoRemoteProvisioningRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                0,
                1,
                vec![],
            ),
            resulting_mode: ResultingMode::CreateProject,
            bypass_paths: vec![
                bypass(BypassPathClass::CreateEmptyWorkspace, Some("Ctrl+Shift+N")),
                bypass(BypassPathClass::OpenFolderWithoutStarter, None),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::PendingEvaluation,
                false,
                false,
                "No managed service boundary",
                "No significant download",
                "None",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove generated project files and restore empty workspace state.",
                "Discard template application and open an empty workspace instead.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn starter_community_create_service() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "starter_community_create_service",
        scenario_label: "Community starter — create service with side effects",
        narrative: "A community-supported starter that creates a microservice, requires network egress to a community registry, and carries explicit trust notes.",
        fixture_filename: "starter_community_create_service.json",
        expected_entry_kind: EntryKind::Starter,
        expected_resulting_mode: ResultingMode::CreateService,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}starter_community_create_service"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "community.starter.node.microservice",
                "Node microservice starter",
                "Scaffolds a Node.js microservice with Express, Docker compose, and community registry dependencies.",
                "3.2.1",
                "aureline://starter/community_node_microservice",
                EntryKind::Starter,
            ),
            source_review: source(
                SourceClass::Community,
                "community_registry",
                "signed_review_required",
                "NodeCommunity",
                "aureline://trust/community_node",
                vec![
                    "This starter is published by a community maintainer, not Aureline.",
                    "Review the Dockerfile and package.json before trusting.",
                ],
            ),
            support_review: support(SupportClass::CommunitySupported, "active"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalWithContainer,
                HostBoundaryClass::HostLocalWithContainerAttached,
                vec!["node", "docker"],
                vec!["darwin-aarch64", "linux-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::NearExpiry, Some(7200), Some(7200), "community_maintainer", "signed_review_required"),
            setup_review: setup(
                vec![
                    SetupActionClass::DownloadDependencyIndex,
                    SetupActionClass::InstallExtension,
                    SetupActionClass::TrustWorkspace,
                ],
                "Two to five minutes",
                true,
                "Network required to community registry and Docker Hub",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::EgressToCommunityOriginUserReviewRequired,
                ExtensionInstallClass::MarketplaceExtensionInstallUserReviewRequired,
                RemoteProvisioningClass::ContainerAttachRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                2,
                3,
                vec![
                    "Downloads Node dependencies from npm registry.",
                    "Pulls a Docker base image from Docker Hub.",
                    "Installs a community VS Code extension for Dockerfile support.",
                ],
            ),
            resulting_mode: ResultingMode::CreateService,
            bypass_paths: vec![
                bypass(BypassPathClass::CreateEmptyWorkspace, None),
                bypass(BypassPathClass::ContinueWithoutStarter, Some("Esc")),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::PendingEvaluation,
                false,
                true,
                "No managed service boundary",
                "Docker image pull and npm install",
                "Docker base image (~200 MB) and npm dependencies (~50 MB)",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove scaffolded service files, node_modules, and Docker volumes.",
                "Discard starter and open an empty workspace.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn prebuild_fresh_resume_live() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "prebuild_fresh_resume_live",
        scenario_label: "Fresh prebuild — resume live workspace",
        narrative: "A fresh prebuild snapshot that can be resumed as a live workspace with no rebuild required.",
        fixture_filename: "prebuild_fresh_resume_live.json",
        expected_entry_kind: EntryKind::Prebuild,
        expected_resulting_mode: ResultingMode::ResumeLiveWorkspace,
        expected_honesty_marker_present: false,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}prebuild_fresh_resume_live"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "aureline.prebuild.rust.devcontainer",
                "Rust devcontainer prebuild",
                "Prebuilt Rust devcontainer with cargo, clippy, rustfmt, and language server indexed.",
                "2026.06.01",
                "aureline://prebuild/rust_devcontainer",
                EntryKind::Prebuild,
            ),
            source_review: source(
                SourceClass::FirstParty,
                "bundled",
                "signed_verified",
                "Aureline",
                "aureline://trust/first_party",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalWithDevcontainer,
                HostBoundaryClass::HostLocalWithDevcontainerAttached,
                vec!["rust"],
                vec!["linux-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::FreshUnderWindow, Some(1800), Some(14400), "first_party_template_pipeline", "signed_verified"),
            setup_review: setup(
                vec![SetupActionClass::RestoreCachedArtifact],
                "Under 30 seconds",
                false,
                "No network required; artifacts are cached locally",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::NoNetworkEgressRequired,
                ExtensionInstallClass::NoExtensionInstallRequired,
                RemoteProvisioningClass::DevcontainerAttachRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                0,
                1,
                vec!["Attaches to a prebuilt devcontainer image stored locally."],
            ),
            resulting_mode: ResultingMode::ResumeLiveWorkspace,
            bypass_paths: vec![
                bypass(BypassPathClass::OpenPrebuildMinimal, None),
                bypass(BypassPathClass::OpenFolderWithoutStarter, Some("Ctrl+O")),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::Trusted,
                false,
                false,
                "No managed service boundary",
                "Local devcontainer attach only",
                "None",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Detach devcontainer and revert to plain folder open.",
                "Open the workspace without devcontainer attach.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn prebuild_stale_start_snapshot() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "prebuild_stale_start_snapshot",
        scenario_label: "Stale prebuild — start from snapshot with revalidation",
        narrative: "A stale prebuild snapshot that requires revalidation before it can be started; the user is told it is stale and given a bypass to clone fresh.",
        fixture_filename: "prebuild_stale_start_snapshot.json",
        expected_entry_kind: EntryKind::Prebuild,
        expected_resulting_mode: ResultingMode::StartFromSnapshot,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 3,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}prebuild_stale_start_snapshot"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "team.prebuild.python.data_science",
                "Python data-science prebuild",
                "Prebuilt Python environment with Jupyter, pandas, and scikit-learn.",
                "2026.05.15",
                "aureline://prebuild/python_data_science",
                EntryKind::Prebuild,
            ),
            source_review: source(
                SourceClass::TeamManaged,
                "team_mirror",
                "signed_rotation_preauthorized",
                "DataScienceTeam",
                "aureline://trust/team_data_science",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalWithContainer,
                HostBoundaryClass::HostLocalWithContainerAttached,
                vec!["python"],
                vec!["darwin-aarch64", "linux-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::StaleOverWindow, Some(86400), Some(36000), "enterprise_mirror_pipeline", "signed_rotation_preauthorized"),
            setup_review: setup(
                vec![
                    SetupActionClass::RestoreCachedArtifact,
                    SetupActionClass::DownloadDependencyIndex,
                ],
                "Five to ten minutes",
                true,
                "Network required to team mirror for dependency revalidation",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::EgressToTeamManagedMirrorOnly,
                ExtensionInstallClass::NoExtensionInstallRequired,
                RemoteProvisioningClass::ContainerAttachRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                0,
                2,
                vec![
                    "Revalidates Python package index against team mirror.",
                    "Rebuilds container layers if cache is invalidated.",
                ],
            ),
            resulting_mode: ResultingMode::StartFromSnapshot,
            bypass_paths: vec![
                bypass(BypassPathClass::CloneRepositoryWithoutStarter, None),
                bypass(BypassPathClass::OpenPrebuildMinimal, None),
                bypass(BypassPathClass::SetUpLater, Some("Esc")),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::Trusted,
                false,
                true,
                "No managed service boundary",
                "Team mirror revalidation",
                "Package index revalidation (~20 MB)",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove rebuilt container layers and restore prior snapshot.",
                "Clone fresh from upstream and skip prebuild reuse.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn prebuild_clone_fresh() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "prebuild_clone_fresh",
        scenario_label: "Prebuild — clone fresh",
        narrative: "A prebuild entry that begins with a fresh clone because no cached snapshot exists; the user sees the clone and setup expectations before committing.",
        fixture_filename: "prebuild_clone_fresh.json",
        expected_entry_kind: EntryKind::Prebuild,
        expected_resulting_mode: ResultingMode::CloneFresh,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}prebuild_clone_fresh"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "aureline.prebuild.go.microservice",
                "Go microservice prebuild",
                "Prebuilt Go environment with modules, delve debugger, and gopls indexed.",
                "2026.06.01",
                "aureline://prebuild/go_microservice",
                EntryKind::Prebuild,
            ),
            source_review: source(
                SourceClass::FirstParty,
                "bundled",
                "signed_verified",
                "Aureline",
                "aureline://trust/first_party",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalOnly,
                HostBoundaryClass::HostLocalDeviceOnly,
                vec!["go"],
                vec!["darwin-aarch64", "linux-x86_64", "windows-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::FreshUnderWindow, Some(0), Some(3600), "first_party_template_pipeline", "signed_verified"),
            setup_review: setup(
                vec![
                    SetupActionClass::DownloadDependencyIndex,
                    SetupActionClass::TrustWorkspace,
                ],
                "Three to seven minutes",
                true,
                "Network required to clone repository and download Go modules",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::EgressToFirstPartyOriginOnly,
                ExtensionInstallClass::NoExtensionInstallRequired,
                RemoteProvisioningClass::NoRemoteProvisioningRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                0,
                2,
                vec!["Clones repository from first-party origin.", "Downloads Go modules from proxy."],
            ),
            resulting_mode: ResultingMode::CloneFresh,
            bypass_paths: vec![
                bypass(BypassPathClass::CloneRepositoryWithoutStarter, None),
                bypass(BypassPathClass::SetUpLater, None),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::PendingEvaluation,
                false,
                false,
                "No managed service boundary",
                "Repository clone and module download",
                "Repository (~5 MB) and modules (~50 MB)",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove cloned repository and module cache.",
                "Clone without prebuild setup and open plain folder.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn template_open_without_starter() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "template_open_without_starter",
        scenario_label: "Template entry with open-without-starter bypass",
        narrative: "A template entry surface that prominently offers an equal-weight 'Open without starter' bypass so the user can open the folder plainly.",
        fixture_filename: "template_open_without_starter.json",
        expected_entry_kind: EntryKind::Template,
        expected_resulting_mode: ResultingMode::OpenWithoutStarter,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}template_open_without_starter"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "aureline.template.typescript.webapp",
                "TypeScript webapp template",
                "Scaffolds a TypeScript webapp with Vite, React, and testing libraries.",
                "2026.06.01",
                "aureline://template/ts_webapp",
                EntryKind::Template,
            ),
            source_review: source(
                SourceClass::FirstParty,
                "bundled",
                "signed_verified",
                "Aureline",
                "aureline://trust/first_party",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalOnly,
                HostBoundaryClass::HostLocalDeviceOnly,
                vec!["typescript", "node"],
                vec!["darwin-aarch64", "linux-x86_64", "windows-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::FreshUnderWindow, Some(0), Some(86400), "first_party_template_pipeline", "signed_verified"),
            setup_review: setup(
                vec![SetupActionClass::DownloadDependencyIndex, SetupActionClass::InstallExtension],
                "Two to four minutes",
                true,
                "Network required to npm registry",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::EgressToFirstPartyOriginOnly,
                ExtensionInstallClass::FirstPartyExtensionInstallRequired,
                RemoteProvisioningClass::NoRemoteProvisioningRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                0,
                2,
                vec!["Downloads npm dependencies.", "Installs first-party TypeScript extension."],
            ),
            resulting_mode: ResultingMode::OpenWithoutStarter,
            bypass_paths: vec![
                bypass(BypassPathClass::OpenFolderWithoutStarter, Some("Ctrl+O")),
                bypass(BypassPathClass::CreateEmptyWorkspace, None),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::PendingEvaluation,
                false,
                false,
                "No managed service boundary",
                "npm dependency download",
                "npm dependencies (~30 MB)",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove node_modules and generated build output.",
                "Open the folder without applying the template.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn starter_failure_partial_apply() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "starter_failure_partial_apply",
        scenario_label: "Starter failure with partial application summary",
        narrative: "A starter run that partially applied: scaffold succeeded, dependency download failed, and cleanup ran; the user sees exactly what remains to review.",
        fixture_filename: "starter_failure_partial_apply.json",
        expected_entry_kind: EntryKind::Starter,
        expected_resulting_mode: ResultingMode::CreateProject,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}starter_failure_partial_apply"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "community.starter.python.flask",
                "Python Flask starter",
                "Scaffolds a Flask application with SQLAlchemy, migrations, and test harness.",
                "2.1.0",
                "aureline://starter/community_python_flask",
                EntryKind::Starter,
            ),
            source_review: source(
                SourceClass::Community,
                "community_registry",
                "signed_review_required",
                "FlaskCommunity",
                "aureline://trust/community_flask",
                vec!["Community starter; review requirements.txt and migration scripts before trusting."],
            ),
            support_review: support(SupportClass::CommunitySupported, "active"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalOnly,
                HostBoundaryClass::HostLocalDeviceOnly,
                vec!["python"],
                vec!["darwin-aarch64", "linux-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::FreshUnderWindow, Some(0), Some(3600), "community_maintainer", "signed_review_required"),
            setup_review: setup(
                vec![
                    SetupActionClass::RunScaffoldHook,
                    SetupActionClass::DownloadDependencyIndex,
                    SetupActionClass::TrustWorkspace,
                ],
                "Three to six minutes",
                true,
                "Network required to PyPI",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::EgressToCommunityOriginUserReviewRequired,
                ExtensionInstallClass::MarketplaceExtensionInstallUserReviewRequired,
                RemoteProvisioningClass::NoRemoteProvisioningRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                1,
                3,
                vec!["Scaffolds project files from template.", "Installs Python packages from PyPI."],
            ),
            resulting_mode: ResultingMode::CreateProject,
            bypass_paths: vec![
                bypass(BypassPathClass::CreateEmptyWorkspace, None),
                bypass(BypassPathClass::ContinueWithoutStarter, None),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::PendingEvaluation,
                false,
                true,
                "No managed service boundary",
                "PyPI package download",
                "Python packages (~15 MB)",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove partially scaffolded files and virtual environment.",
                "Discard partial starter output and create an empty project.",
            ),
            failure_summary: Some(FailureSummary {
                succeeded: vec![failure_item("Project scaffold", FailureOutcomeClass::Succeeded, "All template files written successfully.")],
                skipped: vec![failure_item("Database migration run", FailureOutcomeClass::Skipped, "Skipped because dependency install failed.")],
                partially_applied: vec![failure_item("Dependency install", FailureOutcomeClass::PartiallyApplied, "Flask and SQLAlchemy installed, but greenlet build failed on ARM.")],
                failed: vec![failure_item("Test harness initialization", FailureOutcomeClass::Failed, "Could not write pytest configuration because dependency install was incomplete.")],
                cleanup_ran: vec![failure_item("Partial artifact removal", FailureOutcomeClass::CleanupRan, "Removed broken greenlet build artifacts and incomplete virtual environment.")],
                remaining_user_review: "Review the ARM compatibility of greenlet and retry dependency install, or choose a different starter.".to_string(),
            }),
            support_export: support_meta(),
        },
    }
}

fn prebuild_managed_cloud() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "prebuild_managed_cloud",
        scenario_label: "Managed cloud prebuild with auth and trust boundaries",
        narrative: "A managed-cloud prebuild that requires managed workspace envelope, credential provisioning, and explicit trust review before setup begins.",
        fixture_filename: "prebuild_managed_cloud.json",
        expected_entry_kind: EntryKind::Prebuild,
        expected_resulting_mode: ResultingMode::OpenPrebuildWithSetupActions,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}prebuild_managed_cloud"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "managed.prebuild.enterprise.java",
                "Enterprise Java managed prebuild",
                "Prebuilt managed workspace for enterprise Java with Maven, corporate mirror, and SSO integration.",
                "2026.05.20",
                "aureline://prebuild/enterprise_java_managed",
                EntryKind::Prebuild,
            ),
            source_review: source(
                SourceClass::TeamManaged,
                "enterprise_mirror",
                "signed_verified",
                "EnterprisePlatformTeam",
                "aureline://trust/enterprise_java",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::ManagedCloudRequired,
                HostBoundaryClass::HostManagedWorkspaceRequired,
                vec!["java", "maven"],
                vec!["linux-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::NearExpiry, Some(18000), Some(21600), "managed_workspace_service", "signed_verified"),
            setup_review: setup(
                vec![
                    SetupActionClass::ProvisionRemoteEnvironment,
                    SetupActionClass::AuthenticateToRegistry,
                    SetupActionClass::RestoreCachedArtifact,
                ],
                "Ten to fifteen minutes",
                true,
                "Network required to managed workspace endpoint and corporate Maven mirror",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::EgressToManagedWorkspaceEnvelopeOnly,
                ExtensionInstallClass::ManagedOnlyChannelExtensionInstallRequired,
                RemoteProvisioningClass::ManagedWorkspaceRequired,
                ManagedServiceClass::ManagedWorkspaceEnvelopeRequired,
                CredentialProvisioningClass::CredentialProvisioningStepRequired,
                1,
                4,
                vec![
                    "Provisions managed cloud workspace via enterprise API.",
                    "Authenticates to corporate Maven mirror.",
                    "Installs extensions from managed-only channel.",
                ],
            ),
            resulting_mode: ResultingMode::OpenPrebuildWithSetupActions,
            bypass_paths: vec![
                bypass(BypassPathClass::OpenWorkspaceWithoutStarter, None),
                bypass(BypassPathClass::SetUpLater, Some("Esc")),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::PendingEvaluation,
                true,
                true,
                "Managed workspace envelope required",
                "Managed cloud provisioning and corporate mirror authentication",
                "Managed workspace image (~2 GB) and Maven dependencies (~500 MB)",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Deprovision managed workspace and clear local SSO tokens.",
                "Open workspace locally without managed cloud provisioning.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn template_create_empty() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "template_create_empty",
        scenario_label: "Template with create-empty-workspace bypass",
        narrative: "A template entry where the user chooses to create an empty workspace instead of applying the template; the record reflects the empty-workspace resulting mode.",
        fixture_filename: "template_create_empty.json",
        expected_entry_kind: EntryKind::Template,
        expected_resulting_mode: ResultingMode::CreateEmptyWorkspace,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}template_create_empty"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "aureline.template.cpp.cmake",
                "C++ CMake project template",
                "Scaffolds a C++ project with CMake, vcpkg, and cross-platform build presets.",
                "2026.06.01",
                "aureline://template/cpp_cmake",
                EntryKind::Template,
            ),
            source_review: source(
                SourceClass::FirstParty,
                "bundled",
                "signed_verified",
                "Aureline",
                "aureline://trust/first_party",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalOnly,
                HostBoundaryClass::HostLocalDeviceOnly,
                vec!["cpp", "cmake"],
                vec!["darwin-aarch64", "linux-x86_64", "windows-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::FreshUnderWindow, Some(0), Some(86400), "first_party_template_pipeline", "signed_verified"),
            setup_review: setup(
                vec![SetupActionClass::TrustWorkspace],
                "Under one minute",
                false,
                "No network required",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::NoNetworkEgressRequired,
                ExtensionInstallClass::NoExtensionInstallRequired,
                RemoteProvisioningClass::NoRemoteProvisioningRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                0,
                1,
                vec![],
            ),
            resulting_mode: ResultingMode::CreateEmptyWorkspace,
            bypass_paths: vec![
                bypass(BypassPathClass::CreateEmptyWorkspace, Some("Ctrl+Shift+N")),
                bypass(BypassPathClass::OpenFolderWithoutStarter, None),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::PendingEvaluation,
                false,
                false,
                "No managed service boundary",
                "No significant download",
                "None",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove any accidentally generated files.",
                "Create an empty workspace without template scaffolding.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

fn prebuild_expired_open_minimal() -> TemplateStarterPrebuildEntryScenario {
    TemplateStarterPrebuildEntryScenario {
        scenario_id: "prebuild_expired_open_minimal",
        scenario_label: "Expired prebuild — open minimal",
        narrative: "An expired prebuild that cannot be reused; the user is offered an open-minimal bypass that skips setup actions and opens the workspace plainly.",
        fixture_filename: "prebuild_expired_open_minimal.json",
        expected_entry_kind: EntryKind::Prebuild,
        expected_resulting_mode: ResultingMode::OpenPrebuildMinimal,
        expected_honesty_marker_present: true,
        expected_bypass_path_count: 2,
        input: TemplateStarterPrebuildEntryInput {
            record_id: format!("{CORPUS_RECORD_ID_PREFIX}prebuild_expired_open_minimal"),
            as_of: CORPUS_AS_OF.to_string(),
            accelerator_identity: identity(
                "team.prebuild.ruby.rails",
                "Ruby on Rails prebuild",
                "Prebuilt Ruby on Rails environment with bundler, yarn, and database migration cache.",
                "2026.04.01",
                "aureline://prebuild/ruby_rails",
                EntryKind::Prebuild,
            ),
            source_review: source(
                SourceClass::TeamManaged,
                "team_mirror",
                "signed_rotation_preauthorized",
                "RailsPlatformTeam",
                "aureline://trust/team_rails",
                vec![],
            ),
            support_review: support(SupportClass::OfficiallySupported, "stable"),
            runtime_review: runtime(
                RuntimeScopeClass::LocalWithContainer,
                HostBoundaryClass::HostLocalWithContainerAttached,
                vec!["ruby", "rails"],
                vec!["linux-x86_64"],
            ),
            freshness_review: freshness(FreshnessClass::Expired, Some(259200), Some(86400), "enterprise_mirror_pipeline", "signed_rotation_preauthorized"),
            setup_review: setup(
                vec![SetupActionClass::RestoreCachedArtifact],
                "Not applicable — prebuild expired",
                false,
                "No network required for minimal open",
            ),
            side_effect_envelope: side_effects(
                NetworkEgressClass::NoNetworkEgressRequired,
                ExtensionInstallClass::NoExtensionInstallRequired,
                RemoteProvisioningClass::NoRemoteProvisioningRequired,
                ManagedServiceClass::NoManagedServiceRequired,
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
                0,
                0,
                vec!["Prebuild is expired; minimal open skips all setup actions."],
            ),
            resulting_mode: ResultingMode::OpenPrebuildMinimal,
            bypass_paths: vec![
                bypass(BypassPathClass::OpenFolderWithoutStarter, None),
                bypass(BypassPathClass::SetUpLater, None),
            ],
            trust_auth_boundaries: trust_auth(
                TrustPostureClass::Trusted,
                false,
                false,
                "No managed service boundary",
                "No download for minimal open",
                "None",
            ),
            cleanup_rollback: cleanup(
                true,
                true,
                "Remove expired container image and cached artifacts.",
                "Open the folder without container attach or prebuild setup.",
            ),
            failure_summary: None,
            support_export: support_meta(),
        },
    }
}

// ---------------------------------------------------------------------------
// Public corpus
// ---------------------------------------------------------------------------

/// Returns the full deterministic drill corpus.
pub fn template_starter_prebuild_entry_corpus() -> Vec<TemplateStarterPrebuildEntryScenario> {
    vec![
        template_first_party_create_project(),
        starter_community_create_service(),
        prebuild_fresh_resume_live(),
        prebuild_stale_start_snapshot(),
        prebuild_clone_fresh(),
        template_open_without_starter(),
        starter_failure_partial_apply(),
        prebuild_managed_cloud(),
        template_create_empty(),
        prebuild_expired_open_minimal(),
    ]
}
