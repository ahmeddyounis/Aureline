//! Contract freshness banners, imported-snapshot labels, and refresh, diff, or
//! open-spec flow qualification records.
//!
//! This module owns the typed records that render schema/contract freshness
//! banners anywhere request validation or completion depends on a contract
//! snapshot. Each banner keeps the source service, snapshot date, freshness
//! state, mirror/offline note, and refresh/open-details actions inspectable so
//! GraphQL and other contract-linked requests never let a stale, cached, or
//! imported snapshot masquerade as a live contract. The companion refresh,
//! diff, and open-spec flows let a workspace move off a stale, cached, or
//! imported state without dropping local request context, while preserving the
//! version and snapshot identity that support exports replay.
//!
//! These records reuse the canonical matrix vocabulary
//! ([`ContractKind`], [`ContractSourceClass`], [`ContractFreshnessState`],
//! [`OfflineMirrorBehavior`], [`RequestOriginKind`], [`RetentionMode`]) and the
//! composer export-redaction vocabulary ([`ExportRedactionClass`]) rather than
//! minting local synonyms; the banners are a real consumer of the
//! [`freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix`](crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix)
//! truth and reference it as a verified upstream packet.
//!
//! Raw endpoint URLs, raw secrets, raw request bodies, raw headers, and raw
//! schema payloads do not belong in these records. Banners carry opaque source
//! labels and snapshot digests, closed posture vocabularies, and reviewable
//! summaries. Stale and imported snapshots are always labeled and never appear
//! equivalent to a live contract; freshness is never hidden from browser-
//! companion or managed-request surfaces; refresh never silently falls back to
//! raw execution when contract risk changed; compare flows never force unsafe
//! body or header retention; and managed or companion banners never inherit
//! desktop-local trust or naming.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix::{
    ContractFreshnessState, ContractKind, ContractSourceClass, OfflineMirrorBehavior,
    RequestOriginKind, RetentionMode, API_MATRIX_QUALIFICATION_RECORD_KIND,
};
use crate::implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export::ExportRedactionClass;

/// Supported schema version for freshness-banner qualification packets.
pub const FRESHNESS_BANNER_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`FreshnessBannerQualificationPacket`].
pub const FRESHNESS_BANNER_QUALIFICATION_RECORD_KIND: &str =
    "ship_contract_freshness_banners_imported_snapshot_labels_and_refresh_diff_or_open_spec_flows";

/// Repo-relative path to the checked-in freshness-banner packet.
pub const FRESHNESS_BANNER_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.json";

/// Embedded checked-in packet JSON.
pub const FRESHNESS_BANNER_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.json"
));

/// Qualification label shown on promoted banner surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessBannerQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl FreshnessBannerQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Banner-consumer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessBannerSurfaceKind {
    /// Request composer where requests are edited and sent.
    RequestComposer,
    /// Completion and validation provider that depends on a contract snapshot.
    CompletionProvider,
    /// Browser-companion request surface that can drift from desktop-local state.
    BrowserCompanionSurface,
    /// CLI or headless request execution output.
    CliHeadlessOutput,
    /// Support-export bundle carrying banner truth.
    SupportExport,
    /// Help/About surface describing the banner contract.
    HelpAbout,
}

/// Visual severity class derived from the contract freshness state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BannerSeverityClass {
    /// Live contract; nothing to act on.
    Informational,
    /// Cached but in-window schema; refresh is optional.
    Advisory,
    /// Stale schema; a refresh or diff is recommended before trusting completion.
    StaleWarning,
    /// Imported snapshot; explicitly not live truth.
    ImportedNotice,
    /// No contract is available; validation and completion are unbacked.
    UnavailableBlock,
}

impl BannerSeverityClass {
    /// Returns the severity that a freshness state must map to so banners read
    /// consistently across surfaces.
    pub const fn for_freshness(state: ContractFreshnessState) -> Self {
        match state {
            ContractFreshnessState::LiveContract => Self::Informational,
            ContractFreshnessState::CachedSchema => Self::Advisory,
            ContractFreshnessState::SchemaStale => Self::StaleWarning,
            ContractFreshnessState::ImportedSnapshot => Self::ImportedNotice,
            ContractFreshnessState::ContractUnavailable => Self::UnavailableBlock,
        }
    }
}

/// Action a freshness banner can offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessBannerAction {
    /// Refresh the contract from its source.
    Refresh,
    /// Compare the current snapshot against another snapshot or live.
    Diff,
    /// Open the contract spec inline, in a doc, or in the provider console.
    OpenSpec,
    /// Open banner details (source, snapshot date, freshness, mirror note).
    OpenDetails,
}

/// How a refresh flow re-resolves a contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefreshMode {
    /// Fetch a live contract from the target.
    FetchLive,
    /// Revalidate an existing cached schema against the source.
    RevalidateCache,
    /// Re-import a snapshot from a file or workspace artifact.
    ReimportSnapshot,
}

/// Where an open-spec flow takes the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecTargetKind {
    /// Inline schema/SDL view inside the workspace.
    InlineSchemaView,
    /// External spec document (OpenAPI or SDL file).
    ExternalSpecDoc,
    /// Provider or plugin console that owns the contract.
    ProviderConsole,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessBannerQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable banner surfaces honest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessBannerSurfaceGuardSet {
    /// The freshness banner is visible.
    pub freshness_banner_visible: bool,
    /// The source service label is visible.
    pub source_service_visible: bool,
    /// The snapshot date is visible.
    pub snapshot_date_visible: bool,
    /// The freshness state is visible.
    pub freshness_state_visible: bool,
    /// The mirror/offline note is visible.
    pub mirror_offline_note_visible: bool,
    /// A refresh action is available.
    pub refresh_action_available: bool,
    /// An open-details action is available.
    pub open_details_action_available: bool,
    /// Imported snapshots are explicitly labeled.
    pub imported_snapshot_labeled: bool,
    /// Stale or imported snapshots never look equivalent to a live contract.
    pub stale_not_equivalent_to_live: bool,
    /// Freshness is never hidden from companion or managed surfaces.
    pub companion_managed_freshness_not_hidden: bool,
    /// Refresh never silently falls back to raw execution on contract risk.
    pub no_silent_raw_fallback: bool,
    /// Refresh, diff, and open-spec flows preserve local request context.
    pub local_request_context_preserved: bool,
    /// Companion and managed banners never inherit desktop-local trust.
    pub trust_isolated: bool,
}

impl FreshnessBannerSurfaceGuardSet {
    /// Returns true when every required guard is present.
    pub const fn all_visible(&self) -> bool {
        self.freshness_banner_visible
            && self.source_service_visible
            && self.snapshot_date_visible
            && self.freshness_state_visible
            && self.mirror_offline_note_visible
            && self.refresh_action_available
            && self.open_details_action_available
            && self.imported_snapshot_labeled
            && self.stale_not_equivalent_to_live
            && self.companion_managed_freshness_not_hidden
            && self.no_silent_raw_fallback
            && self.local_request_context_preserved
            && self.trust_isolated
    }
}

/// One governed banner-consumer surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessBannerSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: FreshnessBannerSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: FreshnessBannerQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: FreshnessBannerQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<FreshnessBannerQualificationProof>,
    /// Visible guard set.
    pub guards: FreshnessBannerSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One contract freshness banner row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessBannerRow {
    /// Stable banner id.
    pub banner_id: String,
    /// Owning surface ref.
    pub surface_ref: String,
    /// Contract family the banner describes.
    pub contract_kind: ContractKind,
    /// Matrix contract ref this banner reflects.
    pub matrix_contract_ref: String,
    /// Opaque, non-secret source-service label.
    pub source_service_label: String,
    /// Source class shown on the banner.
    pub source_class: ContractSourceClass,
    /// Freshness state shown on the banner.
    pub freshness_state: ContractFreshnessState,
    /// Severity derived from the freshness state.
    pub severity: BannerSeverityClass,
    /// UTC snapshot date the contract is current as of.
    pub snapshot_date: String,
    /// Request origin this contract resolves against.
    pub origin_kind: RequestOriginKind,
    /// Mirror or offline behavior for the contract source.
    pub mirror_offline_behavior: OfflineMirrorBehavior,
    /// Plain-language mirror/offline note.
    pub mirror_offline_note: String,
    /// Whether validation or completion depends on this contract snapshot.
    pub validation_or_completion_dependent: bool,
    /// Actions the banner offers.
    pub available_actions: Vec<FreshnessBannerAction>,
    /// Whether the source service label is visible.
    pub source_service_visible: bool,
    /// Whether the snapshot date is visible.
    pub snapshot_date_visible: bool,
    /// Whether the freshness state is visible.
    pub freshness_state_visible: bool,
    /// Whether the mirror/offline note is visible.
    pub mirror_offline_note_visible: bool,
    /// Whether an imported snapshot is explicitly labeled.
    pub imported_snapshot_labeled: bool,
    /// Whether the banner may appear equivalent to a live contract.
    pub may_appear_equivalent_to_live: bool,
    /// Whether freshness is hidden on companion or managed surfaces.
    pub hidden_on_companion_or_managed: bool,
    /// Whether the banner inherits desktop-local trust.
    pub inherits_local_trust: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One refresh flow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefreshFlowRow {
    /// Stable flow id.
    pub flow_id: String,
    /// Banner this flow refreshes.
    pub banner_ref: String,
    /// Freshness state the flow starts from.
    pub from_freshness: ContractFreshnessState,
    /// How the flow re-resolves the contract.
    pub refresh_mode: RefreshMode,
    /// Freshness state the flow results in.
    pub result_freshness: ContractFreshnessState,
    /// Whether local request context is preserved across the refresh.
    pub preserves_local_request_context: bool,
    /// Whether local edits are preserved across the refresh.
    pub local_edits_preserved: bool,
    /// Whether retargeting the origin requires an explicit acknowledgement.
    pub requires_origin_ack_on_retarget: bool,
    /// Whether the flow falls back to raw execution on refresh failure.
    pub falls_back_to_raw_on_failure: bool,
    /// Whether drift blocks any silent fallback to raw execution.
    pub drift_blocks_silent_raw_fallback: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One diff/compare flow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffFlowRow {
    /// Stable flow id.
    pub flow_id: String,
    /// Banner this flow compares from.
    pub banner_ref: String,
    /// Opaque base snapshot digest ref.
    pub base_snapshot_ref: String,
    /// Opaque compare snapshot digest ref.
    pub compare_snapshot_ref: String,
    /// Reviewer-facing base version label.
    pub base_version_label: String,
    /// Reviewer-facing compare version label.
    pub compare_version_label: String,
    /// Whether the flow preserves version identity.
    pub preserves_version_identity: bool,
    /// Whether the flow preserves snapshot identity.
    pub preserves_snapshot_identity: bool,
    /// Whether the flow keeps support-export parity.
    pub support_export_parity: bool,
    /// Export redaction posture for the compare export.
    pub export_redaction: ExportRedactionClass,
    /// History retention mode the compare flow relies on.
    pub history_retention_mode: RetentionMode,
    /// Whether the flow forces unsafe body/header retention to compare.
    pub forces_unsafe_body_header_retention: bool,
    /// Whether the flow drops local request context.
    pub drops_local_request_context: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One open-spec flow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenSpecFlowRow {
    /// Stable flow id.
    pub flow_id: String,
    /// Banner this flow opens the spec from.
    pub banner_ref: String,
    /// Where the flow opens the spec.
    pub spec_target: SpecTargetKind,
    /// Opaque snapshot digest ref the flow opens.
    pub snapshot_ref: String,
    /// Whether the flow preserves snapshot identity.
    pub preserves_snapshot_identity: bool,
    /// Whether the flow opens live truth when it is available.
    pub opens_live_when_available: bool,
    /// Whether the flow keeps support-export parity.
    pub support_export_parity: bool,
    /// Whether the flow drops local request context.
    pub drops_local_request_context: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Reference to an upstream M5 packet these banners consume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessBannerUpstreamRefRow {
    /// Stable reference id.
    pub ref_id: String,
    /// Upstream record kind.
    pub upstream_record_kind: String,
    /// Repo-relative path to the upstream packet.
    pub upstream_packet_path: String,
    /// Repo-relative path to the upstream schema.
    pub upstream_schema_path: String,
    /// Whether integration has been verified.
    pub integration_verified: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Summary counts for a freshness-banner qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessBannerQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of banner rows.
    pub banner_count: usize,
    /// Number of refresh-flow rows.
    pub refresh_flow_count: usize,
    /// Number of diff-flow rows.
    pub diff_flow_count: usize,
    /// Number of open-spec flow rows.
    pub open_spec_flow_count: usize,
    /// Number of upstream reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical freshness-banner qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessBannerQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<FreshnessBannerSurfaceQualificationRow>,
    /// Freshness-banner rows.
    pub banners: Vec<FreshnessBannerRow>,
    /// Refresh-flow rows.
    pub refresh_flows: Vec<RefreshFlowRow>,
    /// Diff-flow rows.
    pub diff_flows: Vec<DiffFlowRow>,
    /// Open-spec flow rows.
    pub open_spec_flows: Vec<OpenSpecFlowRow>,
    /// Upstream reference rows.
    pub upstream_refs: Vec<FreshnessBannerUpstreamRefRow>,
    /// Summary counts.
    pub summary: FreshnessBannerQualificationSummary,
}

impl FreshnessBannerQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> FreshnessBannerQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        let integration_pass_count = self
            .upstream_refs
            .iter()
            .filter(|ref_row| ref_row.integration_verified)
            .count();
        FreshnessBannerQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            banner_count: self.banners.len(),
            refresh_flow_count: self.refresh_flows.len(),
            diff_flow_count: self.diff_flows.len(),
            open_spec_flow_count: self.open_spec_flows.len(),
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of banners whose freshness must narrow any live claim
    /// (stale schema or unavailable contract).
    pub fn narrowing_banner_ids(&self) -> Vec<String> {
        self.banners
            .iter()
            .filter(|row| row.freshness_state.narrows_claim())
            .map(|row| row.banner_id.clone())
            .collect()
    }

    /// Returns the ids of banners that carry an imported snapshot, which must be
    /// labeled and never look equivalent to live truth.
    pub fn imported_snapshot_banner_ids(&self) -> Vec<String> {
        self.banners
            .iter()
            .filter(|row| row.freshness_state == ContractFreshnessState::ImportedSnapshot)
            .map(|row| row.banner_id.clone())
            .collect()
    }

    /// Returns the ids of GraphQL banners, the protocol M5 most depends on for
    /// contract-backed completion.
    pub fn graphql_banner_ids(&self) -> Vec<String> {
        self.banners
            .iter()
            .filter(|row| row.contract_kind == ContractKind::Graphql)
            .map(|row| row.banner_id.clone())
            .collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<FreshnessBannerQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != FRESHNESS_BANNER_QUALIFICATION_SCHEMA_VERSION {
            violations.push(FreshnessBannerQualificationViolation::SchemaVersion {
                expected: FRESHNESS_BANNER_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != FRESHNESS_BANNER_QUALIFICATION_RECORD_KIND {
            violations.push(FreshnessBannerQualificationViolation::RecordKind {
                expected: FRESHNESS_BANNER_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let surface_ids = collect_ids(
            self.surfaces.iter().map(|row| row.surface_id.as_str()),
            &mut violations,
            FreshnessBannerViolationKind::Surface,
        );
        let banner_ids = collect_ids(
            self.banners.iter().map(|row| row.banner_id.as_str()),
            &mut violations,
            FreshnessBannerViolationKind::Banner,
        );
        collect_ids(
            self.refresh_flows.iter().map(|row| row.flow_id.as_str()),
            &mut violations,
            FreshnessBannerViolationKind::RefreshFlow,
        );
        collect_ids(
            self.diff_flows.iter().map(|row| row.flow_id.as_str()),
            &mut violations,
            FreshnessBannerViolationKind::DiffFlow,
        );
        collect_ids(
            self.open_spec_flows.iter().map(|row| row.flow_id.as_str()),
            &mut violations,
            FreshnessBannerViolationKind::OpenSpecFlow,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            FreshnessBannerViolationKind::UpstreamRef,
        );

        self.validate_surfaces(&mut violations);
        self.validate_banners(&mut violations, &surface_ids);
        self.validate_refresh_flows(&mut violations, &banner_ids);
        self.validate_diff_flows(&mut violations, &banner_ids);
        self.validate_open_spec_flows(&mut violations, &banner_ids);
        self.validate_upstream_refs(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(FreshnessBannerQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_surfaces(&self, violations: &mut Vec<FreshnessBannerQualificationViolation>) {
        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        FreshnessBannerQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        FreshnessBannerQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    FreshnessBannerQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let surface_kinds: BTreeSet<_> = self.surfaces.iter().map(|row| row.surface_kind).collect();
        for required_kind in [
            FreshnessBannerSurfaceKind::RequestComposer,
            FreshnessBannerSurfaceKind::CompletionProvider,
            FreshnessBannerSurfaceKind::BrowserCompanionSurface,
            FreshnessBannerSurfaceKind::CliHeadlessOutput,
            FreshnessBannerSurfaceKind::SupportExport,
            FreshnessBannerSurfaceKind::HelpAbout,
        ] {
            if !surface_kinds.contains(&required_kind) {
                violations.push(FreshnessBannerQualificationViolation::MissingSurfaceKind {
                    surface_kind: required_kind,
                });
            }
        }
    }

    fn validate_banners(
        &self,
        violations: &mut Vec<FreshnessBannerQualificationViolation>,
        surface_ids: &BTreeSet<String>,
    ) {
        for row in &self.banners {
            if !surface_ids.contains(&row.surface_ref)
                || row.matrix_contract_ref.is_empty()
                || row.source_service_label.is_empty()
                || row.snapshot_date.is_empty()
                || !row.source_service_visible
                || !row.snapshot_date_visible
                || !row.freshness_state_visible
                || !row.mirror_offline_note_visible
            {
                violations.push(FreshnessBannerQualificationViolation::IncompleteBanner {
                    banner_id: row.banner_id.clone(),
                });
            }
            // Severity must be the canonical mapping of the freshness state so
            // banners read the same truth everywhere.
            if row.severity != BannerSeverityClass::for_freshness(row.freshness_state) {
                violations.push(
                    FreshnessBannerQualificationViolation::BannerSeverityMismatch {
                        banner_id: row.banner_id.clone(),
                    },
                );
            }
            // Every banner must offer a refresh and an open-details action.
            if !row
                .available_actions
                .contains(&FreshnessBannerAction::Refresh)
                || !row
                    .available_actions
                    .contains(&FreshnessBannerAction::OpenDetails)
            {
                violations.push(
                    FreshnessBannerQualificationViolation::BannerMissingCoreAction {
                        banner_id: row.banner_id.clone(),
                    },
                );
            }
            // An imported snapshot must be explicitly labeled.
            if row.freshness_state == ContractFreshnessState::ImportedSnapshot
                && !row.imported_snapshot_labeled
            {
                violations.push(
                    FreshnessBannerQualificationViolation::ImportedSnapshotNotLabeled {
                        banner_id: row.banner_id.clone(),
                    },
                );
            }
            // A non-live banner must never appear equivalent to a live contract.
            if row.may_appear_equivalent_to_live {
                violations.push(
                    FreshnessBannerQualificationViolation::BannerMayMasqueradeAsLive {
                        banner_id: row.banner_id.clone(),
                    },
                );
            }
            // Freshness must never be hidden on companion or managed surfaces.
            if row.hidden_on_companion_or_managed {
                violations.push(
                    FreshnessBannerQualificationViolation::CompanionManagedFreshnessHidden {
                        banner_id: row.banner_id.clone(),
                    },
                );
            }
            // Managed and companion origins must never inherit desktop-local trust.
            if row.origin_kind.must_isolate_local_trust() && row.inherits_local_trust {
                violations.push(
                    FreshnessBannerQualificationViolation::BannerInheritsLocalTrust {
                        banner_id: row.banner_id.clone(),
                    },
                );
            }
        }

        let contract_kinds: BTreeSet<_> =
            self.banners.iter().map(|row| row.contract_kind).collect();
        // REST and GraphQL are the protocols M5 claims; GraphQL is the lane this
        // row most depends on. Both must be covered.
        for required_kind in [ContractKind::Rest, ContractKind::Graphql] {
            if !contract_kinds.contains(&required_kind) {
                violations.push(FreshnessBannerQualificationViolation::MissingContractKind {
                    contract_kind: required_kind,
                });
            }
        }

        let freshness_states: BTreeSet<_> =
            self.banners.iter().map(|row| row.freshness_state).collect();
        for required_state in [
            ContractFreshnessState::LiveContract,
            ContractFreshnessState::CachedSchema,
            ContractFreshnessState::SchemaStale,
            ContractFreshnessState::ImportedSnapshot,
            ContractFreshnessState::ContractUnavailable,
        ] {
            if !freshness_states.contains(&required_state) {
                violations.push(
                    FreshnessBannerQualificationViolation::MissingFreshnessState {
                        freshness_state: required_state,
                    },
                );
            }
        }

        // Validation and completion surfaces must carry at least one banner that
        // depends on a contract snapshot, so banners appear anywhere completion
        // or validation depends on a contract.
        for required_surface in [
            FreshnessBannerSurfaceKind::RequestComposer,
            FreshnessBannerSurfaceKind::CompletionProvider,
        ] {
            let covered = self.banners.iter().any(|banner| {
                banner.validation_or_completion_dependent
                    && self.surfaces.iter().any(|surface| {
                        surface.surface_id == banner.surface_ref
                            && surface.surface_kind == required_surface
                    })
            });
            if !covered {
                violations.push(
                    FreshnessBannerQualificationViolation::DependentSurfaceMissingBanner {
                        surface_kind: required_surface,
                    },
                );
            }
        }

        // Companion and managed surfaces that can drift from desktop-local state
        // must carry at least one freshness banner, so freshness is never hidden
        // from them.
        let companion_managed_covered = self.banners.iter().any(|banner| {
            matches!(
                banner.origin_kind,
                RequestOriginKind::Managed | RequestOriginKind::BrowserCompanion
            )
        });
        if !companion_managed_covered {
            violations
                .push(FreshnessBannerQualificationViolation::CompanionManagedOriginNotCovered);
        }
    }

    fn validate_refresh_flows(
        &self,
        violations: &mut Vec<FreshnessBannerQualificationViolation>,
        banner_ids: &BTreeSet<String>,
    ) {
        for row in &self.refresh_flows {
            if !banner_ids.contains(&row.banner_ref)
                || !row.preserves_local_request_context
                || !row.local_edits_preserved
            {
                violations.push(
                    FreshnessBannerQualificationViolation::IncompleteRefreshFlow {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
            // Refresh must never silently fall back to raw execution when
            // contract risk changed.
            if row.falls_back_to_raw_on_failure || !row.drift_blocks_silent_raw_fallback {
                violations.push(
                    FreshnessBannerQualificationViolation::RefreshSilentRawFallback {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
        }

        let modes: BTreeSet<_> = self
            .refresh_flows
            .iter()
            .map(|row| row.refresh_mode)
            .collect();
        for required_mode in [
            RefreshMode::FetchLive,
            RefreshMode::RevalidateCache,
            RefreshMode::ReimportSnapshot,
        ] {
            if !modes.contains(&required_mode) {
                violations.push(FreshnessBannerQualificationViolation::MissingRefreshMode {
                    refresh_mode: required_mode,
                });
            }
        }
    }

    fn validate_diff_flows(
        &self,
        violations: &mut Vec<FreshnessBannerQualificationViolation>,
        banner_ids: &BTreeSet<String>,
    ) {
        for row in &self.diff_flows {
            if !banner_ids.contains(&row.banner_ref)
                || row.base_snapshot_ref.is_empty()
                || row.compare_snapshot_ref.is_empty()
                || row.base_snapshot_ref == row.compare_snapshot_ref
                || row.base_version_label.is_empty()
                || row.compare_version_label.is_empty()
                || !row.preserves_version_identity
                || !row.preserves_snapshot_identity
                || !row.support_export_parity
            {
                violations.push(FreshnessBannerQualificationViolation::IncompleteDiffFlow {
                    flow_id: row.flow_id.clone(),
                });
            }
            // Compare UX must never force unsafe body/header retention by default
            // or drop local request context.
            if row.forces_unsafe_body_header_retention
                || row.drops_local_request_context
                || row.history_retention_mode == RetentionMode::OptInFullCapture
            {
                violations.push(
                    FreshnessBannerQualificationViolation::DiffForcesUnsafeRetention {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_open_spec_flows(
        &self,
        violations: &mut Vec<FreshnessBannerQualificationViolation>,
        banner_ids: &BTreeSet<String>,
    ) {
        for row in &self.open_spec_flows {
            if !banner_ids.contains(&row.banner_ref)
                || row.snapshot_ref.is_empty()
                || !row.preserves_snapshot_identity
                || !row.support_export_parity
                || row.drops_local_request_context
            {
                violations.push(
                    FreshnessBannerQualificationViolation::IncompleteOpenSpecFlow {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
        }

        let targets: BTreeSet<_> = self
            .open_spec_flows
            .iter()
            .map(|row| row.spec_target)
            .collect();
        for required_target in [
            SpecTargetKind::InlineSchemaView,
            SpecTargetKind::ExternalSpecDoc,
            SpecTargetKind::ProviderConsole,
        ] {
            if !targets.contains(&required_target) {
                violations.push(FreshnessBannerQualificationViolation::MissingSpecTarget {
                    spec_target: required_target,
                });
            }
        }
    }

    fn validate_upstream_refs(&self, violations: &mut Vec<FreshnessBannerQualificationViolation>) {
        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(
                    FreshnessBannerQualificationViolation::IncompleteUpstreamRef {
                        ref_id: row.ref_id.clone(),
                    },
                );
            }
        }
        // The banners must consume the frozen API-collection matrix as a verified
        // upstream packet.
        let consumes_matrix = self.upstream_refs.iter().any(|row| {
            row.upstream_record_kind == API_MATRIX_QUALIFICATION_RECORD_KIND
                && row.integration_verified
        });
        if !consumes_matrix {
            violations.push(FreshnessBannerQualificationViolation::MatrixUpstreamNotIntegrated);
        }
    }
}

/// Loads the checked-in freshness-banner qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_freshness_banner_qualification(
) -> Result<FreshnessBannerQualificationPacket, serde_json::Error> {
    serde_json::from_str(FRESHNESS_BANNER_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessBannerViolationKind {
    /// Surface rows.
    Surface,
    /// Banner rows.
    Banner,
    /// Refresh-flow rows.
    RefreshFlow,
    /// Diff-flow rows.
    DiffFlow,
    /// Open-spec flow rows.
    OpenSpecFlow,
    /// Upstream reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<FreshnessBannerQualificationViolation>,
    kind: FreshnessBannerViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(FreshnessBannerQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for freshness-banner qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FreshnessBannerQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: FreshnessBannerViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required consumer surface kind is missing.
    MissingSurfaceKind {
        surface_kind: FreshnessBannerSurfaceKind,
    },
    /// Banner row does not project source/snapshot/freshness truth everywhere.
    IncompleteBanner { banner_id: String },
    /// Banner severity does not match its freshness state.
    BannerSeverityMismatch { banner_id: String },
    /// Banner does not offer the required refresh and open-details actions.
    BannerMissingCoreAction { banner_id: String },
    /// Imported-snapshot banner is not explicitly labeled.
    ImportedSnapshotNotLabeled { banner_id: String },
    /// Banner may masquerade a non-live snapshot as a live contract.
    BannerMayMasqueradeAsLive { banner_id: String },
    /// Freshness is hidden on a companion or managed surface.
    CompanionManagedFreshnessHidden { banner_id: String },
    /// Banner inherits desktop-local trust it must not have.
    BannerInheritsLocalTrust { banner_id: String },
    /// Required contract kind is missing.
    MissingContractKind { contract_kind: ContractKind },
    /// Required freshness state is missing.
    MissingFreshnessState {
        freshness_state: ContractFreshnessState,
    },
    /// A validation/completion surface carries no contract-dependent banner.
    DependentSurfaceMissingBanner {
        surface_kind: FreshnessBannerSurfaceKind,
    },
    /// No banner covers a companion or managed origin.
    CompanionManagedOriginNotCovered,
    /// Refresh flow does not preserve local request context or resolve its banner.
    IncompleteRefreshFlow { flow_id: String },
    /// Refresh flow may fall back to raw execution silently.
    RefreshSilentRawFallback { flow_id: String },
    /// Required refresh mode is missing.
    MissingRefreshMode { refresh_mode: RefreshMode },
    /// Diff flow does not preserve version/snapshot identity or export parity.
    IncompleteDiffFlow { flow_id: String },
    /// Diff flow forces unsafe retention or drops local request context.
    DiffForcesUnsafeRetention { flow_id: String },
    /// Open-spec flow does not preserve snapshot identity or export parity.
    IncompleteOpenSpecFlow { flow_id: String },
    /// Required open-spec target is missing.
    MissingSpecTarget { spec_target: SpecTargetKind },
    /// Upstream reference is incomplete.
    IncompleteUpstreamRef { ref_id: String },
    /// The banners do not consume the API-collection matrix as a verified upstream packet.
    MatrixUpstreamNotIntegrated,
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for FreshnessBannerQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingSurfaceKind { surface_kind } => {
                write!(f, "consumer surface kind {surface_kind:?} is not covered")
            }
            Self::IncompleteBanner { banner_id } => {
                write!(f, "{banner_id} does not project banner truth everywhere")
            }
            Self::BannerSeverityMismatch { banner_id } => {
                write!(f, "{banner_id} severity does not match its freshness state")
            }
            Self::BannerMissingCoreAction { banner_id } => {
                write!(f, "{banner_id} lacks a refresh or open-details action")
            }
            Self::ImportedSnapshotNotLabeled { banner_id } => {
                write!(f, "{banner_id} carries an unlabeled imported snapshot")
            }
            Self::BannerMayMasqueradeAsLive { banner_id } => {
                write!(
                    f,
                    "{banner_id} may masquerade non-live schema as live truth"
                )
            }
            Self::CompanionManagedFreshnessHidden { banner_id } => {
                write!(
                    f,
                    "{banner_id} hides freshness on a companion or managed surface"
                )
            }
            Self::BannerInheritsLocalTrust { banner_id } => {
                write!(
                    f,
                    "{banner_id} inherits desktop-local trust it must not have"
                )
            }
            Self::MissingContractKind { contract_kind } => {
                write!(f, "contract kind {contract_kind:?} is not covered")
            }
            Self::MissingFreshnessState { freshness_state } => {
                write!(f, "freshness state {freshness_state:?} is not covered")
            }
            Self::DependentSurfaceMissingBanner { surface_kind } => {
                write!(
                    f,
                    "surface kind {surface_kind:?} carries no contract-dependent banner"
                )
            }
            Self::CompanionManagedOriginNotCovered => {
                write!(f, "no banner covers a companion or managed origin")
            }
            Self::IncompleteRefreshFlow { flow_id } => {
                write!(f, "{flow_id} does not preserve local request context")
            }
            Self::RefreshSilentRawFallback { flow_id } => {
                write!(f, "{flow_id} may fall back to raw execution silently")
            }
            Self::MissingRefreshMode { refresh_mode } => {
                write!(f, "refresh mode {refresh_mode:?} is not covered")
            }
            Self::IncompleteDiffFlow { flow_id } => {
                write!(
                    f,
                    "{flow_id} does not preserve version or snapshot identity"
                )
            }
            Self::DiffForcesUnsafeRetention { flow_id } => {
                write!(
                    f,
                    "{flow_id} forces unsafe retention or drops local context"
                )
            }
            Self::IncompleteOpenSpecFlow { flow_id } => {
                write!(
                    f,
                    "{flow_id} does not preserve snapshot identity or export parity"
                )
            }
            Self::MissingSpecTarget { spec_target } => {
                write!(f, "open-spec target {spec_target:?} is not covered")
            }
            Self::IncompleteUpstreamRef { ref_id } => {
                write!(
                    f,
                    "{ref_id} does not project upstream reference truth everywhere"
                )
            }
            Self::MatrixUpstreamNotIntegrated => {
                write!(
                    f,
                    "banners do not consume the API-collection matrix as a verified upstream packet"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for FreshnessBannerQualificationViolation {}
