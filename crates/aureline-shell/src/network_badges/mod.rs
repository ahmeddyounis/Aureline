//! Beta network-badge audit projection for network-capable surfaces.
//!
//! Every network-capable beta surface (self-update, AI broker / prompt /
//! tool-call, connected providers, extension marketplace / install, and docs /
//! help packs) must disclose three stable badges: the [`EgressClass`] of the
//! actual traffic, the [`OriginScopeClass`] of the actor that issued it, and
//! the [`LocalityClass`] of where the request is routed. This module promotes
//! that disclosure into a single inspectable beta page covering the connected,
//! mirror-only, offline, and enterprise-managed profiles.
//!
//! The page is consumed by the admin/settings center, support export wrapper,
//! shell network strip, headless inspector, and reviewer fixtures. None of
//! those surfaces re-derive a local `is_public` or `is_mirrored` boolean; they
//! read [`seeded_network_badge_beta_page`] by reference.
//!
//! The page intentionally fails closed:
//!
//! - locality must align with egress class (a `mirrored` locality cannot be
//!   served by `public_internet` egress, and `public_cloud` locality must
//!   carry an explainer that names the public route);
//! - bindings must not permit an undeclared public-cloud fallback;
//! - raw secret or private-key material is excluded from the record.
//!
//! The reviewer-facing landing page is
//! [`/docs/ux/m3/network_egress_truth_beta.md`](../../../../docs/ux/m3/network_egress_truth_beta.md).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Beta schema version exported with every network-badge record.
pub const NETWORK_BADGE_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta network-badge record.
pub const NETWORK_BADGE_BETA_SHARED_CONTRACT_REF: &str = "network:network_badges_beta:v1";

/// Stable record kind for [`NetworkBadgeBetaPage`] payloads.
pub const NETWORK_BADGE_BETA_PAGE_RECORD_KIND: &str = "network_network_badges_beta_page_record";

/// Stable record kind for [`NetworkBadgeBetaRow`] payloads.
pub const NETWORK_BADGE_BETA_ROW_RECORD_KIND: &str = "network_network_badges_beta_row_record";

/// Stable record kind for [`NetworkBadgeBetaProfileBinding`] payloads.
pub const NETWORK_BADGE_BETA_PROFILE_BINDING_RECORD_KIND: &str =
    "network_network_badges_beta_profile_binding_record";

/// Stable record kind for [`NetworkBadgeBetaSupportRow`] payloads.
pub const NETWORK_BADGE_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "network_network_badges_beta_support_row_record";

/// Stable record kind for [`NetworkBadgeBetaDefect`] payloads.
pub const NETWORK_BADGE_BETA_DEFECT_RECORD_KIND: &str = "network_network_badges_beta_defect_record";

/// Stable record kind for [`NetworkBadgeBetaSupportExport`] payloads.
pub const NETWORK_BADGE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "network_network_badges_beta_support_export_record";

/// Stable record kind for [`NetworkBadgeBetaSummary`] payloads.
pub const NETWORK_BADGE_BETA_SUMMARY_RECORD_KIND: &str =
    "network_network_badges_beta_summary_record";

/// Stable record kind for [`NetworkBadgeBetaRenderSummary`] payloads.
pub const NETWORK_BADGE_BETA_RENDER_RECORD_KIND: &str = "shell_network_badges_beta_render_record";

/// Source matrix this beta projection consumes.
pub const NETWORK_BADGE_BETA_SOURCE_MATRIX_REF: &str = "artifacts/network/proxy_lab_matrix.yaml";

/// Egress class shared across desktop, CLI, support exports, policy bundles,
/// and admin surfaces.
///
/// The vocabulary mirrors the TDD §7.11.5 enum and never widens without the
/// security council reviewing the addition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressClass {
    /// Request is satisfied entirely on the local machine.
    LocalOnly,
    /// Request reaches a target-local endpoint (e.g. tunneled remote-attach).
    TargetLocal,
    /// Request reaches an org-approved external endpoint.
    OrgApprovedExternal,
    /// Request reaches the public internet.
    PublicInternet,
    /// Request reaches a signed mirror only; public fallback is forbidden.
    MirrorOnly,
    /// Request is denied outright on this profile.
    DenyAll,
}

impl EgressClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::TargetLocal => "target_local",
            Self::OrgApprovedExternal => "org_approved_external",
            Self::PublicInternet => "public_internet",
            Self::MirrorOnly => "mirror_only",
            Self::DenyAll => "deny_all",
        }
    }

    /// True when the class permits traffic to leave the local machine.
    pub const fn permits_egress(self) -> bool {
        !matches!(self, Self::LocalOnly | Self::DenyAll)
    }
}

/// Origin scope shared across UI, CLI, support exports, and admin surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginScopeClass {
    /// Request originates on the desktop client running the IDE.
    DesktopClient,
    /// Request originates from a remote-attach target.
    RemoteTarget,
    /// Request originates from a managed service acting on the user's behalf.
    ManagedService,
    /// Request originates inside an extension host.
    ExtensionHost,
    /// Request originates from a headless / CI runner.
    HeadlessRunner,
}

impl OriginScopeClass {
    /// All required origin scopes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::DesktopClient,
        Self::RemoteTarget,
        Self::ManagedService,
        Self::ExtensionHost,
        Self::HeadlessRunner,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopClient => "desktop_client",
            Self::RemoteTarget => "remote_target",
            Self::ManagedService => "managed_service",
            Self::ExtensionHost => "extension_host",
            Self::HeadlessRunner => "headless_runner",
        }
    }
}

/// Locality of a routed network action.
///
/// Locality is the high-level disclosure the user reads on the badge. It
/// answers the question "where might this data leave the machine to?" using a
/// vocabulary that maps cleanly onto policy and procurement language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityClass {
    /// Action stays on the local machine.
    LocalOnly,
    /// Action is routed through a signed mirror.
    Mirrored,
    /// Action reaches a customer-hosted (self-hosted) origin.
    SelfHosted,
    /// Action reaches a vendor-managed service.
    Managed,
    /// Action reaches the public cloud (third-party SaaS or public endpoint).
    PublicCloud,
}

impl LocalityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Mirrored => "mirrored",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::PublicCloud => "public_cloud",
        }
    }

    /// True when traffic leaves the local machine.
    pub const fn leaves_local_machine(self) -> bool {
        !matches!(self, Self::LocalOnly)
    }

    /// Returns the egress classes that may carry this locality.
    pub fn allowed_egress_classes(self) -> &'static [EgressClass] {
        match self {
            Self::LocalOnly => &[EgressClass::LocalOnly, EgressClass::DenyAll],
            Self::Mirrored => &[EgressClass::MirrorOnly],
            Self::SelfHosted => &[EgressClass::OrgApprovedExternal, EgressClass::TargetLocal],
            Self::Managed => &[
                EgressClass::PublicInternet,
                EgressClass::OrgApprovedExternal,
            ],
            Self::PublicCloud => &[EgressClass::PublicInternet],
        }
    }
}

/// Network-capable beta surface that must carry the badge trio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkBadgeSurfaceClass {
    /// Self-update and signed-mirror refresh surface.
    Update,
    /// AI broker, prompt, and tool-call surface.
    Ai,
    /// Connected-provider and remote-attach surface.
    Provider,
    /// Extension marketplace, registry, and install surface.
    Extension,
    /// Docs pack and help-search surface.
    DocsHelp,
}

impl NetworkBadgeSurfaceClass {
    /// All required network-capable surfaces in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Update,
        Self::Ai,
        Self::Provider,
        Self::Extension,
        Self::DocsHelp,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Update => "update",
            Self::Ai => "ai",
            Self::Provider => "provider",
            Self::Extension => "extension",
            Self::DocsHelp => "docs_help",
        }
    }
}

/// Connectedness or enterprise profile under which a row is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkBadgeBetaProfileClass {
    /// Normal connected beta profile.
    Connected,
    /// Mirror-only profile where public endpoints are not fallback targets.
    MirrorOnly,
    /// Offline or air-gapped profile.
    Offline,
    /// Enterprise-managed profile with signed managed policy narrowing.
    EnterpriseManaged,
}

impl NetworkBadgeBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Typed defect kind for the network-badge beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkBadgeBetaDefectKind {
    /// A required network-capable surface is absent from the page.
    MissingSurfaceCoverage,
    /// A row is missing a required profile binding.
    MissingProfileCoverage,
    /// A row's surface token does not match its surface class.
    SurfaceTokenDrift,
    /// A binding's egress-class token does not match its egress class.
    EgressClassTokenDrift,
    /// A binding's origin-scope token does not match its origin scope.
    OriginScopeTokenDrift,
    /// A binding's locality token does not match its locality class.
    LocalityTokenDrift,
    /// Locality is inconsistent with the declared egress class.
    LocalityInconsistentWithEgress,
    /// A binding routes to public cloud without naming the public route.
    HiddenPublicCloudRouting,
    /// A binding permits an undeclared public-cloud fallback.
    HiddenPublicEndpointFallback,
    /// A binding's `explainer_label` is empty.
    EmptyExplainerLabel,
    /// The support row drifted from the live row vocabulary.
    SupportRowVocabularyDrift,
    /// A row leaks raw secret or private-key material.
    RawSecretOrPrivateMaterialExposed,
}

impl NetworkBadgeBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingSurfaceCoverage => "missing_surface_coverage",
            Self::MissingProfileCoverage => "missing_profile_coverage",
            Self::SurfaceTokenDrift => "surface_token_drift",
            Self::EgressClassTokenDrift => "egress_class_token_drift",
            Self::OriginScopeTokenDrift => "origin_scope_token_drift",
            Self::LocalityTokenDrift => "locality_token_drift",
            Self::LocalityInconsistentWithEgress => "locality_inconsistent_with_egress",
            Self::HiddenPublicCloudRouting => "hidden_public_cloud_routing",
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::EmptyExplainerLabel => "empty_explainer_label",
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
            Self::RawSecretOrPrivateMaterialExposed => "raw_secret_or_private_material_exposed",
        }
    }
}

/// One profile binding for a beta network-badge row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaProfileBinding {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Profile class.
    pub profile_class: NetworkBadgeBetaProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_token: String,
    /// Egress class for this profile binding.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Origin scope that issues the action.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Locality of the routed action.
    pub locality: LocalityClass,
    /// Stable token for [`Self::locality`].
    pub locality_token: String,
    /// Reviewable explainer the badge surfaces alongside the trio.
    pub explainer_label: String,
    /// Reviewable route label (mirror artifact, self-hosted host, etc.).
    pub route_label: String,
    /// Optional reviewable pointer to a signed route or policy artifact.
    pub route_attribution_ref: String,
    /// True when no undeclared public endpoint fallback is permitted.
    pub no_public_endpoint_fallback: bool,
}

/// One live beta row covering one network-capable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Network-capable surface class.
    pub surface: NetworkBadgeSurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Source matrix anchor this beta row implements.
    pub source_matrix_ref: String,
    /// Profile bindings (one per profile).
    pub profile_bindings: Vec<NetworkBadgeBetaProfileBinding>,
    /// Export-safe summary shown in support packets.
    pub support_export_summary: String,
    /// True when no undeclared public endpoint fallback is allowed on any binding.
    pub no_public_endpoint_fallback: bool,
    /// True when raw secret / private-key material is excluded from the record.
    pub raw_secret_or_private_material_excluded: bool,
}

/// Export-safe support row aligned one-to-one with a live beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaSupportRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Live row id.
    pub row_id: String,
    /// Surface token copied from the live row.
    pub surface_token: String,
    /// Per-profile egress-class tokens.
    pub egress_class_tokens_by_profile: BTreeMap<String, String>,
    /// Per-profile origin-scope tokens.
    pub origin_scope_tokens_by_profile: BTreeMap<String, String>,
    /// Per-profile locality tokens.
    pub locality_tokens_by_profile: BTreeMap<String, String>,
    /// Per-profile explainer labels (export-safe, no raw secrets).
    pub explainer_labels_by_profile: BTreeMap<String, String>,
    /// Export-safe support summary.
    pub support_export_summary: String,
    /// True when no undeclared public endpoint fallback is allowed.
    pub no_public_endpoint_fallback: bool,
    /// True when raw secret / private-key material is excluded.
    pub raw_secret_or_private_material_excluded: bool,
}

impl NetworkBadgeBetaSupportRow {
    /// Builds an export-safe row from a live beta row.
    pub fn from_row(row: &NetworkBadgeBetaRow) -> Self {
        let mut egress_class_tokens_by_profile = BTreeMap::new();
        let mut origin_scope_tokens_by_profile = BTreeMap::new();
        let mut locality_tokens_by_profile = BTreeMap::new();
        let mut explainer_labels_by_profile = BTreeMap::new();
        for binding in &row.profile_bindings {
            egress_class_tokens_by_profile.insert(
                binding.profile_token.clone(),
                binding.egress_class_token.clone(),
            );
            origin_scope_tokens_by_profile.insert(
                binding.profile_token.clone(),
                binding.origin_scope_token.clone(),
            );
            locality_tokens_by_profile.insert(
                binding.profile_token.clone(),
                binding.locality_token.clone(),
            );
            explainer_labels_by_profile.insert(
                binding.profile_token.clone(),
                binding.explainer_label.clone(),
            );
        }
        Self {
            record_kind: NETWORK_BADGE_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: NETWORK_BADGE_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_BADGE_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: row.row_id.clone(),
            surface_token: row.surface_token.clone(),
            egress_class_tokens_by_profile,
            origin_scope_tokens_by_profile,
            locality_tokens_by_profile,
            explainer_labels_by_profile,
            support_export_summary: row.support_export_summary.clone(),
            no_public_endpoint_fallback: row.no_public_endpoint_fallback,
            raw_secret_or_private_material_excluded: row.raw_secret_or_private_material_excluded,
        }
    }
}

/// Typed validation defect for the network-badge beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: NetworkBadgeBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Row id, or `page` for page-level defects.
    pub row_id: String,
    /// Surface token, or `page` for page-level defects.
    pub surface_token: String,
    /// Profile token, or `*` when not bound to a single profile.
    pub profile_token: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl NetworkBadgeBetaDefect {
    fn new(
        defect_kind: NetworkBadgeBetaDefectKind,
        row_id: impl Into<String>,
        surface_token: impl Into<String>,
        profile_token: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: NETWORK_BADGE_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: NETWORK_BADGE_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_BADGE_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            row_id: row_id.into(),
            surface_token: surface_token.into(),
            profile_token: profile_token.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the network-badge beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of live rows.
    pub row_count: usize,
    /// Number of support rows.
    pub support_row_count: usize,
    /// Surface tokens present on the page.
    pub surfaces_present: Vec<String>,
    /// Profile tokens present on every valid row.
    pub profiles_present: Vec<String>,
    /// Egress-class tokens present across the page.
    pub egress_class_tokens_present: Vec<String>,
    /// Origin-scope tokens present across the page.
    pub origin_scope_tokens_present: Vec<String>,
    /// Locality tokens present across the page.
    pub locality_tokens_present: Vec<String>,
    /// Number of bindings whose locality leaves the local machine.
    pub bindings_leaving_local_machine_count: usize,
    /// Number of bindings whose egress is `deny_all`.
    pub bindings_deny_all_count: usize,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl NetworkBadgeBetaSummary {
    /// Builds a summary from live rows, support rows, and defects.
    pub fn from_rows(
        rows: &[NetworkBadgeBetaRow],
        support_rows: &[NetworkBadgeBetaSupportRow],
        defects: &[NetworkBadgeBetaDefect],
    ) -> Self {
        let surfaces_present: BTreeSet<String> =
            rows.iter().map(|row| row.surface_token.clone()).collect();
        let profiles_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| row.profile_bindings.iter().map(|b| b.profile_token.clone()))
            .collect();
        let egress_class_tokens_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| {
                row.profile_bindings
                    .iter()
                    .map(|b| b.egress_class_token.clone())
            })
            .collect();
        let origin_scope_tokens_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| {
                row.profile_bindings
                    .iter()
                    .map(|b| b.origin_scope_token.clone())
            })
            .collect();
        let locality_tokens_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| {
                row.profile_bindings
                    .iter()
                    .map(|b| b.locality_token.clone())
            })
            .collect();
        let mut bindings_leaving_local_machine_count = 0_usize;
        let mut bindings_deny_all_count = 0_usize;
        for binding in rows.iter().flat_map(|row| row.profile_bindings.iter()) {
            if binding.locality.leaves_local_machine() {
                bindings_leaving_local_machine_count += 1;
            }
            if binding.egress_class == EgressClass::DenyAll {
                bindings_deny_all_count += 1;
            }
        }
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            page_record_kind: NETWORK_BADGE_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: NETWORK_BADGE_BETA_SUMMARY_RECORD_KIND.to_owned(),
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            surfaces_present: surfaces_present.into_iter().collect(),
            profiles_present: profiles_present.into_iter().collect(),
            egress_class_tokens_present: egress_class_tokens_present.into_iter().collect(),
            origin_scope_tokens_present: origin_scope_tokens_present.into_iter().collect(),
            locality_tokens_present: locality_tokens_present.into_iter().collect(),
            bindings_leaving_local_machine_count,
            bindings_deny_all_count,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by admin/settings center, support export,
/// shell summary, headless inspector, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Live beta rows.
    pub rows: Vec<NetworkBadgeBetaRow>,
    /// Support/export rows.
    pub support_rows: Vec<NetworkBadgeBetaSupportRow>,
    /// Typed validation defects.
    pub defects: Vec<NetworkBadgeBetaDefect>,
    /// Aggregate summary.
    pub summary: NetworkBadgeBetaSummary,
}

/// Support-export wrapper for the network-badge beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: NetworkBadgeBetaPage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw secret / private-key material is excluded.
    pub raw_secret_or_private_material_excluded: bool,
}

impl NetworkBadgeBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: NetworkBadgeBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: NETWORK_BADGE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NETWORK_BADGE_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_BADGE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_secret_or_private_material_excluded: true,
        }
    }
}

/// Shell-facing rendering summary for the beta network-badge page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkBadgeBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of rows rendered.
    pub row_count: usize,
    /// Surface tokens rendered by the shell.
    pub surfaces_present: Vec<String>,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Egress-class tokens rendered by the shell.
    pub egress_class_tokens_present: Vec<String>,
    /// Origin-scope tokens rendered by the shell.
    pub origin_scope_tokens_present: Vec<String>,
    /// Locality tokens rendered by the shell.
    pub locality_tokens_present: Vec<String>,
    /// Number of bindings whose locality leaves the local machine.
    pub bindings_leaving_local_machine_count: usize,
    /// Number of bindings whose egress is `deny_all`.
    pub bindings_deny_all_count: usize,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl NetworkBadgeBetaRenderSummary {
    /// Builds the shell render summary from the beta page.
    pub fn from_page(page: &NetworkBadgeBetaPage) -> Self {
        Self {
            record_kind: NETWORK_BADGE_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: NETWORK_BADGE_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_BADGE_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count: page.summary.row_count,
            surfaces_present: page.summary.surfaces_present.clone(),
            profiles_present: page.summary.profiles_present.clone(),
            egress_class_tokens_present: page.summary.egress_class_tokens_present.clone(),
            origin_scope_tokens_present: page.summary.origin_scope_tokens_present.clone(),
            locality_tokens_present: page.summary.locality_tokens_present.clone(),
            bindings_leaving_local_machine_count: page.summary.bindings_leaving_local_machine_count,
            bindings_deny_all_count: page.summary.bindings_deny_all_count,
            defect_count: page.summary.defect_count,
        }
    }
}

/// Builds the seeded network-badge beta page covering every network-capable
/// surface across the connected, mirror-only, offline, and enterprise-managed
/// profiles.
pub fn seeded_network_badge_beta_page() -> NetworkBadgeBetaPage {
    let rows: Vec<NetworkBadgeBetaRow> = NetworkBadgeSurfaceClass::ALL
        .iter()
        .copied()
        .map(seed_row)
        .collect();
    let support_rows: Vec<NetworkBadgeBetaSupportRow> = rows
        .iter()
        .map(NetworkBadgeBetaSupportRow::from_row)
        .collect();
    let defects = audit_network_badge_beta_rows(&rows, &support_rows);
    let summary = NetworkBadgeBetaSummary::from_rows(&rows, &support_rows, &defects);
    NetworkBadgeBetaPage {
        record_kind: NETWORK_BADGE_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: NETWORK_BADGE_BETA_SCHEMA_VERSION,
        shared_contract_ref: NETWORK_BADGE_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: NETWORK_BADGE_BETA_SOURCE_MATRIX_REF.to_owned(),
        rows,
        support_rows,
        defects,
        summary,
    }
}

/// Validates a beta page and returns typed defects on failure.
pub fn validate_network_badge_beta_page(
    page: &NetworkBadgeBetaPage,
) -> Result<(), Vec<NetworkBadgeBetaDefect>> {
    let defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for network-badge beta rows and support rows.
pub fn audit_network_badge_beta_rows(
    rows: &[NetworkBadgeBetaRow],
    support_rows: &[NetworkBadgeBetaSupportRow],
) -> Vec<NetworkBadgeBetaDefect> {
    let mut defects = Vec::new();
    let expected_surfaces: BTreeSet<&str> = NetworkBadgeSurfaceClass::ALL
        .iter()
        .map(|surface| surface.as_str())
        .collect();
    let observed_surfaces: BTreeSet<&str> =
        rows.iter().map(|row| row.surface_token.as_str()).collect();
    for missing in expected_surfaces.difference(&observed_surfaces) {
        defects.push(NetworkBadgeBetaDefect::new(
            NetworkBadgeBetaDefectKind::MissingSurfaceCoverage,
            "page",
            *missing,
            "*",
            "surface",
            "claimed network-capable surface is missing from the page",
        ));
    }

    let support_by_row: BTreeMap<&str, &NetworkBadgeBetaSupportRow> = support_rows
        .iter()
        .map(|support| (support.row_id.as_str(), support))
        .collect();

    for row in rows {
        if row.surface_token != row.surface.as_str() {
            defects.push(row_defect(
                row,
                "*",
                NetworkBadgeBetaDefectKind::SurfaceTokenDrift,
                "surface_token",
                "surface_token must match surface class",
            ));
        }

        if !row.no_public_endpoint_fallback {
            defects.push(row_defect(
                row,
                "*",
                NetworkBadgeBetaDefectKind::HiddenPublicEndpointFallback,
                "no_public_endpoint_fallback",
                "row permits undeclared public endpoint fallback",
            ));
        }

        if !row.raw_secret_or_private_material_excluded {
            defects.push(row_defect(
                row,
                "*",
                NetworkBadgeBetaDefectKind::RawSecretOrPrivateMaterialExposed,
                "raw_secret_or_private_material_excluded",
                "network-badge rows must be export-safe metadata",
            ));
        }

        let observed_profiles: BTreeSet<&str> = row
            .profile_bindings
            .iter()
            .map(|binding| binding.profile_token.as_str())
            .collect();
        for expected in NetworkBadgeBetaProfileClass::ALL {
            if !observed_profiles.contains(expected.as_str()) {
                defects.push(row_defect(
                    row,
                    expected.as_str(),
                    NetworkBadgeBetaDefectKind::MissingProfileCoverage,
                    "profile_bindings",
                    format!("missing {} profile binding", expected.as_str()),
                ));
            }
        }

        for binding in &row.profile_bindings {
            if binding.egress_class_token != binding.egress_class.as_str() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkBadgeBetaDefectKind::EgressClassTokenDrift,
                    "egress_class_token",
                    "egress_class_token must match egress_class",
                ));
            }
            if binding.origin_scope_token != binding.origin_scope.as_str() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkBadgeBetaDefectKind::OriginScopeTokenDrift,
                    "origin_scope_token",
                    "origin_scope_token must match origin_scope",
                ));
            }
            if binding.locality_token != binding.locality.as_str() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkBadgeBetaDefectKind::LocalityTokenDrift,
                    "locality_token",
                    "locality_token must match locality",
                ));
            }
            if !binding
                .locality
                .allowed_egress_classes()
                .contains(&binding.egress_class)
            {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkBadgeBetaDefectKind::LocalityInconsistentWithEgress,
                    "locality",
                    format!(
                        "locality {} is not consistent with egress class {}",
                        binding.locality.as_str(),
                        binding.egress_class.as_str(),
                    ),
                ));
            }
            if binding.locality == LocalityClass::PublicCloud
                && binding.route_label.trim().is_empty()
            {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkBadgeBetaDefectKind::HiddenPublicCloudRouting,
                    "route_label",
                    "public-cloud locality must name the public route",
                ));
            }
            if !binding.no_public_endpoint_fallback {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkBadgeBetaDefectKind::HiddenPublicEndpointFallback,
                    "no_public_endpoint_fallback",
                    "profile binding permits undeclared public endpoint fallback",
                ));
            }
            if binding.explainer_label.trim().is_empty() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkBadgeBetaDefectKind::EmptyExplainerLabel,
                    "explainer_label",
                    "every binding must carry a reviewable explainer",
                ));
            }
        }

        match support_by_row.get(row.row_id.as_str()) {
            Some(support) => compare_support_row(row, support, &mut defects),
            None => defects.push(row_defect(
                row,
                "*",
                NetworkBadgeBetaDefectKind::SupportRowVocabularyDrift,
                "support_rows",
                "missing support row for live network-badge row",
            )),
        }
    }

    defects
}

fn compare_support_row(
    row: &NetworkBadgeBetaRow,
    support: &NetworkBadgeBetaSupportRow,
    defects: &mut Vec<NetworkBadgeBetaDefect>,
) {
    let mut egress: BTreeMap<String, String> = BTreeMap::new();
    let mut origins: BTreeMap<String, String> = BTreeMap::new();
    let mut locality: BTreeMap<String, String> = BTreeMap::new();
    let mut labels: BTreeMap<String, String> = BTreeMap::new();
    for binding in &row.profile_bindings {
        egress.insert(
            binding.profile_token.clone(),
            binding.egress_class_token.clone(),
        );
        origins.insert(
            binding.profile_token.clone(),
            binding.origin_scope_token.clone(),
        );
        locality.insert(
            binding.profile_token.clone(),
            binding.locality_token.clone(),
        );
        labels.insert(
            binding.profile_token.clone(),
            binding.explainer_label.clone(),
        );
    }
    if support.surface_token != row.surface_token
        || support.egress_class_tokens_by_profile != egress
        || support.origin_scope_tokens_by_profile != origins
        || support.locality_tokens_by_profile != locality
        || support.explainer_labels_by_profile != labels
        || support.support_export_summary != row.support_export_summary
        || support.no_public_endpoint_fallback != row.no_public_endpoint_fallback
        || support.raw_secret_or_private_material_excluded
            != row.raw_secret_or_private_material_excluded
    {
        defects.push(row_defect(
            row,
            "*",
            NetworkBadgeBetaDefectKind::SupportRowVocabularyDrift,
            "support_row",
            "support/export row drifted from live row vocabulary",
        ));
    }
}

fn row_defect(
    row: &NetworkBadgeBetaRow,
    profile_token: impl Into<String>,
    kind: NetworkBadgeBetaDefectKind,
    field: impl Into<String>,
    note: impl Into<String>,
) -> NetworkBadgeBetaDefect {
    NetworkBadgeBetaDefect::new(
        kind,
        row.row_id.clone(),
        row.surface_token.clone(),
        profile_token,
        field,
        note,
    )
}

fn seed_row(surface: NetworkBadgeSurfaceClass) -> NetworkBadgeBetaRow {
    let bindings: Vec<NetworkBadgeBetaProfileBinding> = NetworkBadgeBetaProfileClass::ALL
        .iter()
        .copied()
        .map(|profile| seed_binding(surface, profile))
        .collect();
    let support_export_summary = format!(
        "{}: connected={}, mirror_only={}, offline={}, enterprise_managed={}; no public fallback; raw secrets excluded.",
        surface.as_str(),
        bindings[0].locality_token,
        bindings[1].locality_token,
        bindings[2].locality_token,
        bindings[3].locality_token,
    );
    NetworkBadgeBetaRow {
        record_kind: NETWORK_BADGE_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: NETWORK_BADGE_BETA_SCHEMA_VERSION,
        shared_contract_ref: NETWORK_BADGE_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: format!("network_badges_beta:{}", surface.as_str()),
        surface,
        surface_token: surface.as_str().to_owned(),
        source_matrix_ref: format!(
            "{}#{}",
            NETWORK_BADGE_BETA_SOURCE_MATRIX_REF,
            surface.as_str()
        ),
        profile_bindings: bindings,
        support_export_summary,
        no_public_endpoint_fallback: true,
        raw_secret_or_private_material_excluded: true,
    }
}

fn seed_binding(
    surface: NetworkBadgeSurfaceClass,
    profile: NetworkBadgeBetaProfileClass,
) -> NetworkBadgeBetaProfileBinding {
    let (egress, origin, locality, explainer, route_label, route_ref) =
        seed_binding_fields(surface, profile);
    NetworkBadgeBetaProfileBinding {
        record_kind: NETWORK_BADGE_BETA_PROFILE_BINDING_RECORD_KIND.to_owned(),
        schema_version: NETWORK_BADGE_BETA_SCHEMA_VERSION,
        shared_contract_ref: NETWORK_BADGE_BETA_SHARED_CONTRACT_REF.to_owned(),
        profile_class: profile,
        profile_token: profile.as_str().to_owned(),
        egress_class: egress,
        egress_class_token: egress.as_str().to_owned(),
        origin_scope: origin,
        origin_scope_token: origin.as_str().to_owned(),
        locality,
        locality_token: locality.as_str().to_owned(),
        explainer_label: explainer.to_owned(),
        route_label: route_label.to_owned(),
        route_attribution_ref: route_ref.to_owned(),
        no_public_endpoint_fallback: true,
    }
}

fn seed_binding_fields(
    surface: NetworkBadgeSurfaceClass,
    profile: NetworkBadgeBetaProfileClass,
) -> (
    EgressClass,
    OriginScopeClass,
    LocalityClass,
    &'static str,
    &'static str,
    &'static str,
) {
    match (surface, profile) {
        // Update
        (NetworkBadgeSurfaceClass::Update, NetworkBadgeBetaProfileClass::Connected) => (
            EgressClass::PublicInternet,
            OriginScopeClass::DesktopClient,
            LocalityClass::Managed,
            "Self-update reaches the vendor-managed update service over the public internet; releases are signed and verified before apply.",
            "vendor-managed update channel (updates.aureline.dev)",
            "artifacts/network/signatures/managed-update-channel.sig",
        ),
        (NetworkBadgeSurfaceClass::Update, NetworkBadgeBetaProfileClass::MirrorOnly) => (
            EgressClass::MirrorOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::Mirrored,
            "Self-update reaches a signed mirror only; public update endpoints are not a fallback target.",
            "signed mirror route (mirror.corp.example/aureline-updates)",
            "artifacts/network/signatures/mirror-update-route.sig",
        ),
        (NetworkBadgeSurfaceClass::Update, NetworkBadgeBetaProfileClass::Offline) => (
            EgressClass::LocalOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::LocalOnly,
            "Self-update is offline; releases are applied from a locally imported, signed bundle only.",
            "air-gapped signed transfer (local update bundle)",
            "artifacts/network/signatures/airgap-update-bundle.sig",
        ),
        (NetworkBadgeSurfaceClass::Update, NetworkBadgeBetaProfileClass::EnterpriseManaged) => (
            EgressClass::OrgApprovedExternal,
            OriginScopeClass::DesktopClient,
            LocalityClass::SelfHosted,
            "Self-update reaches a customer-hosted update endpoint approved by admin policy; route is signed.",
            "self-hosted update endpoint (updates.corp.example)",
            "artifacts/network/signatures/managed-update-route.sig",
        ),

        // AI
        (NetworkBadgeSurfaceClass::Ai, NetworkBadgeBetaProfileClass::Connected) => (
            EgressClass::PublicInternet,
            OriginScopeClass::DesktopClient,
            LocalityClass::Managed,
            "AI broker reaches the vendor-managed AI gateway over the public internet; prompts and tool calls are governed by the AI policy.",
            "vendor-managed AI gateway (ai.aureline.dev)",
            "artifacts/network/signatures/managed-ai-gateway.sig",
        ),
        (NetworkBadgeSurfaceClass::Ai, NetworkBadgeBetaProfileClass::MirrorOnly) => (
            EgressClass::MirrorOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::Mirrored,
            "AI broker reaches a signed mirrored AI gateway only; public AI endpoints are not a fallback target.",
            "signed mirror AI gateway (mirror.corp.example/ai)",
            "artifacts/network/signatures/mirror-ai-gateway.sig",
        ),
        (NetworkBadgeSurfaceClass::Ai, NetworkBadgeBetaProfileClass::Offline) => (
            EgressClass::DenyAll,
            OriginScopeClass::DesktopClient,
            LocalityClass::LocalOnly,
            "AI broker is denied on the offline profile; AI-bearing actions are not available.",
            "",
            "",
        ),
        (NetworkBadgeSurfaceClass::Ai, NetworkBadgeBetaProfileClass::EnterpriseManaged) => (
            EgressClass::OrgApprovedExternal,
            OriginScopeClass::DesktopClient,
            LocalityClass::SelfHosted,
            "AI broker reaches a customer-hosted AI gateway approved by admin policy; route is signed and tool-call narrowing applies.",
            "self-hosted AI gateway (ai.corp.example)",
            "artifacts/network/signatures/managed-ai-route.sig",
        ),

        // Provider
        (NetworkBadgeSurfaceClass::Provider, NetworkBadgeBetaProfileClass::Connected) => (
            EgressClass::PublicInternet,
            OriginScopeClass::DesktopClient,
            LocalityClass::PublicCloud,
            "Connected provider reaches the third-party SaaS over the public internet; auth posture and tool calls are recorded per action.",
            "third-party SaaS (e.g. api.github.com)",
            "",
        ),
        (NetworkBadgeSurfaceClass::Provider, NetworkBadgeBetaProfileClass::MirrorOnly) => (
            EgressClass::MirrorOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::Mirrored,
            "Connected provider reaches a signed mirrored provider only; public provider endpoints are not a fallback target.",
            "signed mirror provider route (mirror.corp.example/git)",
            "artifacts/network/signatures/mirror-provider-route.sig",
        ),
        (NetworkBadgeSurfaceClass::Provider, NetworkBadgeBetaProfileClass::Offline) => (
            EgressClass::DenyAll,
            OriginScopeClass::DesktopClient,
            LocalityClass::LocalOnly,
            "Connected provider is denied on the offline profile; remote-attach and provider tool calls are not available.",
            "",
            "",
        ),
        (NetworkBadgeSurfaceClass::Provider, NetworkBadgeBetaProfileClass::EnterpriseManaged) => (
            EgressClass::OrgApprovedExternal,
            OriginScopeClass::DesktopClient,
            LocalityClass::SelfHosted,
            "Connected provider reaches a customer-hosted provider gateway approved by admin policy; route is signed.",
            "self-hosted provider gateway (git.corp.example)",
            "artifacts/network/signatures/managed-provider-route.sig",
        ),

        // Extension
        (NetworkBadgeSurfaceClass::Extension, NetworkBadgeBetaProfileClass::Connected) => (
            EgressClass::PublicInternet,
            OriginScopeClass::DesktopClient,
            LocalityClass::PublicCloud,
            "Extension marketplace and install reach the public registry over the public internet; manifests and packages are signed and verified before activation.",
            "public extension registry (extensions.aureline.dev)",
            "artifacts/network/signatures/public-extension-registry.sig",
        ),
        (NetworkBadgeSurfaceClass::Extension, NetworkBadgeBetaProfileClass::MirrorOnly) => (
            EgressClass::MirrorOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::Mirrored,
            "Extension marketplace and install reach a signed mirrored registry only; public extension endpoints are not a fallback target.",
            "signed mirror extension registry (mirror.corp.example/extensions)",
            "artifacts/network/signatures/mirror-extension-registry.sig",
        ),
        (NetworkBadgeSurfaceClass::Extension, NetworkBadgeBetaProfileClass::Offline) => (
            EgressClass::LocalOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::LocalOnly,
            "Extension install is offline; only locally imported, signed extension bundles can be activated.",
            "manual signed extension import",
            "artifacts/network/signatures/airgap-extension-bundle.sig",
        ),
        (NetworkBadgeSurfaceClass::Extension, NetworkBadgeBetaProfileClass::EnterpriseManaged) => (
            EgressClass::OrgApprovedExternal,
            OriginScopeClass::DesktopClient,
            LocalityClass::SelfHosted,
            "Extension marketplace and install reach a customer-hosted registry approved by admin policy; route is signed.",
            "self-hosted extension registry (extensions.corp.example)",
            "artifacts/network/signatures/managed-extension-registry.sig",
        ),

        // Docs / help
        (NetworkBadgeSurfaceClass::DocsHelp, NetworkBadgeBetaProfileClass::Connected) => (
            EgressClass::PublicInternet,
            OriginScopeClass::DesktopClient,
            LocalityClass::Managed,
            "Docs pack and help search reach the vendor-managed docs service over the public internet; payloads carry no workspace content.",
            "vendor-managed docs service (docs.aureline.dev)",
            "artifacts/network/signatures/managed-docs-route.sig",
        ),
        (NetworkBadgeSurfaceClass::DocsHelp, NetworkBadgeBetaProfileClass::MirrorOnly) => (
            EgressClass::MirrorOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::Mirrored,
            "Docs pack and help search reach a signed mirrored docs route only; public docs endpoints are not a fallback target.",
            "signed mirror docs route (mirror.corp.example/docs)",
            "artifacts/network/signatures/mirror-docs-route.sig",
        ),
        (NetworkBadgeSurfaceClass::DocsHelp, NetworkBadgeBetaProfileClass::Offline) => (
            EgressClass::LocalOnly,
            OriginScopeClass::DesktopClient,
            LocalityClass::LocalOnly,
            "Docs pack and help search are offline; only the bundled, signed docs pack is available.",
            "bundled signed docs pack",
            "artifacts/network/signatures/airgap-docs-pack.sig",
        ),
        (NetworkBadgeSurfaceClass::DocsHelp, NetworkBadgeBetaProfileClass::EnterpriseManaged) => (
            EgressClass::OrgApprovedExternal,
            OriginScopeClass::DesktopClient,
            LocalityClass::SelfHosted,
            "Docs pack and help search reach a customer-hosted docs route approved by admin policy; route is signed.",
            "self-hosted docs route (docs.corp.example)",
            "artifacts/network/signatures/managed-docs-route.sig",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_covers_every_surface_and_profile_with_zero_defects() {
        let page = seeded_network_badge_beta_page();
        assert_eq!(page.rows.len(), NetworkBadgeSurfaceClass::ALL.len());
        assert_eq!(page.support_rows.len(), page.rows.len());
        assert!(
            page.defects.is_empty(),
            "seeded page must carry no defects, got {:?}",
            page.defects
        );
        validate_network_badge_beta_page(&page).expect("seeded page validates");
        for surface in NetworkBadgeSurfaceClass::ALL {
            let row = page
                .rows
                .iter()
                .find(|row| row.surface == surface)
                .expect("row");
            assert_eq!(
                row.profile_bindings.len(),
                NetworkBadgeBetaProfileClass::ALL.len()
            );
        }
        assert!(page
            .summary
            .surfaces_present
            .contains(&"docs_help".to_owned()));
        assert!(page
            .summary
            .profiles_present
            .contains(&"mirror_only".to_owned()));
        assert!(page
            .summary
            .egress_class_tokens_present
            .contains(&"mirror_only".to_owned()));
        assert!(page
            .summary
            .locality_tokens_present
            .contains(&"public_cloud".to_owned()));
        assert!(page
            .summary
            .origin_scope_tokens_present
            .contains(&"desktop_client".to_owned()));
    }

    #[test]
    fn validator_rejects_locality_inconsistent_with_egress() {
        let mut page = seeded_network_badge_beta_page();
        let binding = page.rows[0]
            .profile_bindings
            .iter_mut()
            .find(|b| b.profile_class == NetworkBadgeBetaProfileClass::Connected)
            .expect("binding");
        // Connected update normally has locality=managed; flip to mirrored without changing egress.
        binding.locality = LocalityClass::Mirrored;
        binding.locality_token = LocalityClass::Mirrored.as_str().to_owned();
        let defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == NetworkBadgeBetaDefectKind::LocalityInconsistentWithEgress));
    }

    #[test]
    fn validator_rejects_hidden_public_cloud_routing() {
        let mut page = seeded_network_badge_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.surface == NetworkBadgeSurfaceClass::Provider)
            .expect("provider row");
        let binding = row
            .profile_bindings
            .iter_mut()
            .find(|b| b.profile_class == NetworkBadgeBetaProfileClass::Connected)
            .expect("binding");
        binding.route_label.clear();
        let defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(
            |defect| defect.defect_kind == NetworkBadgeBetaDefectKind::HiddenPublicCloudRouting
        ));
    }

    #[test]
    fn validator_rejects_support_row_drift() {
        let mut page = seeded_network_badge_beta_page();
        page.support_rows[0]
            .locality_tokens_by_profile
            .insert("connected".to_owned(), "drifted_locality".to_owned());
        let defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == NetworkBadgeBetaDefectKind::SupportRowVocabularyDrift));
    }

    #[test]
    fn validator_rejects_missing_surface() {
        let mut page = seeded_network_badge_beta_page();
        page.rows
            .retain(|row| row.surface != NetworkBadgeSurfaceClass::DocsHelp);
        page.support_rows
            .retain(|row| row.surface_token != "docs_help");
        let defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
        assert!(
            defects
                .iter()
                .any(|defect| defect.defect_kind
                    == NetworkBadgeBetaDefectKind::MissingSurfaceCoverage)
        );
    }

    #[test]
    fn validator_rejects_hidden_public_endpoint_fallback() {
        let mut page = seeded_network_badge_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.surface == NetworkBadgeSurfaceClass::Update)
            .expect("update row");
        row.no_public_endpoint_fallback = false;
        let binding = row
            .profile_bindings
            .iter_mut()
            .find(|b| b.profile_class == NetworkBadgeBetaProfileClass::MirrorOnly)
            .expect("binding");
        binding.no_public_endpoint_fallback = false;
        let defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == NetworkBadgeBetaDefectKind::HiddenPublicEndpointFallback));
    }

    #[test]
    fn validator_rejects_empty_explainer_label() {
        let mut page = seeded_network_badge_beta_page();
        let binding = page.rows[0]
            .profile_bindings
            .iter_mut()
            .find(|b| b.profile_class == NetworkBadgeBetaProfileClass::Connected)
            .expect("binding");
        binding.explainer_label.clear();
        let defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind == NetworkBadgeBetaDefectKind::EmptyExplainerLabel));
    }

    #[test]
    fn support_export_marks_raw_material_excluded() {
        let page = seeded_network_badge_beta_page();
        let export = NetworkBadgeBetaSupportExport::from_page(
            "test-export:network-badges:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_secret_or_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }

    #[test]
    fn render_summary_mirrors_page_counts() {
        let page = seeded_network_badge_beta_page();
        let summary = NetworkBadgeBetaRenderSummary::from_page(&page);
        assert_eq!(summary.row_count, page.summary.row_count);
        assert_eq!(
            summary.bindings_leaving_local_machine_count,
            page.summary.bindings_leaving_local_machine_count
        );
        assert_eq!(
            summary.bindings_deny_all_count,
            page.summary.bindings_deny_all_count
        );
        assert_eq!(summary.defect_count, 0);
    }
}
