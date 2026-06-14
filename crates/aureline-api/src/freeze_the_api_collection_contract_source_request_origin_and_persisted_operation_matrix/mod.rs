//! API-collection, contract-source, request-origin, and persisted-operation
//! matrix qualification records.
//!
//! This module owns the typed records that keep operation collections, request
//! lists, schema/contract source badges, freshness states, request origins,
//! persisted-operation bindings, and retention modes attributable and
//! inspectable across REST, GraphQL, and plugin-owned contract rows. The matrix
//! locks the closed vocabulary for live contract, cached schema, schema stale,
//! imported snapshot, origin changed, persisted-operation drift, and contract
//! unavailable so request workspaces, CLI/headless output, diagnostics,
//! support exports, and certification scorecards read the same truth.
//!
//! Raw endpoint URLs, raw secrets, raw request bodies, raw headers, and raw
//! schema payloads do not belong in these records. Rows carry stable IDs,
//! closed posture vocabularies, opaque refs, and reviewable summaries that UI,
//! CLI, export, support, and public-proof surfaces can ingest safely. Request
//! files stay text-first and versionable; request history never retains raw
//! bodies or headers by default; persisted-operation and schema drift never
//! fall back silently to raw execution; and browser-companion or managed
//! origins never inherit desktop-local trust or naming.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for API-collection matrix qualification packets.
pub const API_MATRIX_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ApiMatrixQualificationPacket`].
pub const API_MATRIX_QUALIFICATION_RECORD_KIND: &str =
    "freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix";

/// Repo-relative path to the checked-in API-collection matrix packet.
pub const API_MATRIX_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json";

/// Embedded checked-in packet JSON.
pub const API_MATRIX_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json"
));

/// Qualification label shown on promoted matrix-consumer surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiMatrixQualificationLabel {
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

impl ApiMatrixQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Matrix-consumer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiMatrixSurfaceKind {
    /// Request workspace collection tree, request list, and badges.
    RequestWorkspace,
    /// CLI or headless request execution output.
    CliHeadlessOutput,
    /// Diagnostics surface reporting contract and origin drift.
    DiagnosticsSurface,
    /// Support-export bundle carrying matrix truth.
    SupportExport,
    /// Certification scorecard ingesting the matrix.
    CertificationScorecard,
}

/// Contract family the matrix row describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractKind {
    /// REST contract (OpenAPI or equivalent description).
    Rest,
    /// GraphQL contract (introspection schema or SDL).
    Graphql,
    /// Plugin-owned contract supplied by an extension.
    PluginOwned,
}

/// Where the contract/schema badge says the description came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractSourceClass {
    /// Live contract fetched from the target.
    LiveContract,
    /// Cached schema served from a previous fetch.
    CachedSchema,
    /// Imported snapshot loaded from a file or workspace artifact.
    ImportedSnapshot,
    /// Plugin-provided contract owned by an extension.
    PluginProvided,
    /// No contract is available for the target.
    ContractUnavailable,
}

/// Freshness vocabulary locked across schema/contract badges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractFreshnessState {
    /// Contract is live and current as of the last send.
    LiveContract,
    /// Schema is cached and within its freshness window.
    CachedSchema,
    /// Schema is stale and explicitly labeled as such.
    SchemaStale,
    /// Schema is an imported snapshot, not live truth.
    ImportedSnapshot,
    /// No contract truth is available.
    ContractUnavailable,
}

impl ContractFreshnessState {
    /// Returns true when the freshness state must narrow any claim that depends
    /// on the contract being live.
    pub const fn narrows_claim(self) -> bool {
        matches!(self, Self::SchemaStale | Self::ContractUnavailable)
    }
}

/// Request-origin lane the matrix row targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestOriginKind {
    /// Localhost or loopback target on the desktop.
    LocalHost,
    /// Remote host reached over the network.
    Remote,
    /// Container or compose service name.
    Container,
    /// Managed-workspace or cloud-hosted target.
    Managed,
    /// Browser-companion runtime target.
    BrowserCompanion,
}

impl RequestOriginKind {
    /// Returns true when the origin must never inherit desktop-local trust or
    /// naming assumptions.
    pub const fn must_isolate_local_trust(self) -> bool {
        matches!(self, Self::Managed | Self::BrowserCompanion)
    }
}

/// Whether a request origin has drifted from its last resolved target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestOriginDriftState {
    /// Origin resolves to the same target as before.
    OriginStable,
    /// Origin changed since the request was last resolved.
    OriginChanged,
}

/// Binding state of a persisted-operation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistedOperationBindingState {
    /// Operation is raw text and not bound to a persisted id.
    NotPersisted,
    /// Operation is bound to a current persisted id.
    BoundCurrent,
    /// Persisted-operation id no longer matches the operation text.
    PersistedOperationDrift,
}

/// How a request-list entry carries its operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestBindingKind {
    /// Entry holds raw, locally editable request text.
    RawLocalText,
    /// Entry holds a persisted-operation binding, not raw text.
    PersistedOperation,
}

/// Scope a retention class applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionScope {
    /// Request collection definitions (text-first, versionable).
    Collection,
    /// Request execution history.
    History,
}

/// Retention mode shown for a collection or history class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionMode {
    /// Request definitions kept as versionable text.
    TextFirstVersioned,
    /// History keeps method/url/status/timing metadata only.
    MetadataOnly,
    /// History keeps redacted bodies for replay.
    RedactedReplayable,
    /// Full body and header capture is opt-in only.
    OptInFullCapture,
}

/// Offline or mirror behavior for a retention class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineMirrorBehavior {
    /// A mirror copy is maintained for offline access.
    MirrorMaintained,
    /// Behavior degrades to read-only when offline.
    OfflineDegraded,
    /// No mirror is kept.
    NoMirror,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiMatrixQualificationProof {
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

/// Boolean guard set that keeps stable surfaces from inheriting generic truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiMatrixSurfaceGuardSet {
    /// Operation collections are visible.
    pub collection_visible: bool,
    /// The request list is visible.
    pub request_list_visible: bool,
    /// Contract-source badges are visible.
    pub contract_source_visible: bool,
    /// Schema freshness state is visible.
    pub schema_freshness_visible: bool,
    /// Request origin and drift are visible.
    pub request_origin_visible: bool,
    /// Persisted-operation binding state is visible.
    pub persisted_operation_visible: bool,
    /// Retention posture is visible.
    pub retention_posture_visible: bool,
    /// Browser-companion and managed origins are isolated from local trust.
    pub origin_trust_isolated: bool,
    /// Drift never silently falls back to raw request execution.
    pub no_silent_raw_fallback: bool,
}

impl ApiMatrixSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.collection_visible
            && self.request_list_visible
            && self.contract_source_visible
            && self.schema_freshness_visible
            && self.request_origin_visible
            && self.persisted_operation_visible
            && self.retention_posture_visible
            && self.origin_trust_isolated
            && self.no_silent_raw_fallback
    }
}

/// One governed matrix-consumer surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiMatrixSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: ApiMatrixSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: ApiMatrixQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: ApiMatrixQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<ApiMatrixQualificationProof>,
    /// Visible guard set.
    pub guards: ApiMatrixSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One contract row covering source and freshness truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractRow {
    /// Stable contract id.
    pub contract_id: String,
    /// Contract family.
    pub contract_kind: ContractKind,
    /// Source class shown on the badge.
    pub source_class: ContractSourceClass,
    /// Freshness state.
    pub freshness_state: ContractFreshnessState,
    /// Opaque schema digest ref (not the raw schema).
    pub digest_ref: String,
    /// Request-origin row this contract resolves against.
    pub origin_ref: String,
    /// Persisted-operation binding ref, if any.
    pub persisted_operation_ref: Option<String>,
    /// Whether the source badge is visible.
    pub source_badge_visible: bool,
    /// Whether the freshness state is visible.
    pub freshness_visible: bool,
    /// Whether stale or unavailable contracts are explicitly labeled.
    pub stale_labeled: bool,
    /// Whether stale or imported schema may masquerade as live truth.
    pub may_masquerade_as_live: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One operation-collection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiCollectionRow {
    /// Stable collection id.
    pub collection_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Request-list member refs.
    pub request_refs: Vec<String>,
    /// Contract refs referenced by the collection.
    pub contract_refs: Vec<String>,
    /// Retention class ref governing the collection.
    pub retention_class_ref: String,
    /// Whether request definitions stay text-first and versionable.
    pub text_first_versioned: bool,
    /// Whether the collection is diffable.
    pub diffable: bool,
    /// Whether the collection is reusable from CLI and automation.
    pub cli_reusable: bool,
}

/// One request-list entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestListRow {
    /// Stable request id.
    pub request_id: String,
    /// Owning collection ref.
    pub collection_ref: String,
    /// Contract family the request targets.
    pub contract_kind: ContractKind,
    /// Request-origin ref.
    pub origin_ref: String,
    /// How the request carries its operation.
    pub binding_kind: RequestBindingKind,
    /// Persisted-operation binding ref when bound; absent for raw text.
    pub persisted_operation_ref: Option<String>,
    /// Whether the entry holds raw, locally editable text.
    pub raw_local_text: bool,
    /// Method or operation label for display.
    pub method_or_operation_label: String,
    /// Whether the entry is diffable.
    pub diffable: bool,
}

/// One request-origin row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestOriginRow {
    /// Stable origin id.
    pub origin_id: String,
    /// Origin lane.
    pub origin_kind: RequestOriginKind,
    /// Opaque, non-secret display label for the target.
    pub target_label: String,
    /// Drift state.
    pub drift_state: RequestOriginDriftState,
    /// Whether the origin inherits desktop-local trust.
    pub inherits_local_trust: bool,
    /// Whether the origin keeps an explicit, named target.
    pub explicit_naming: bool,
    /// Whether silent retargeting is blocked behind an acknowledgement.
    pub retarget_requires_ack: bool,
    /// Browser-runtime trust-class ref, if any.
    pub trust_class_ref: Option<String>,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One persisted-operation binding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOperationBindingRow {
    /// Stable binding id.
    pub binding_id: String,
    /// Operation ref this binding describes.
    pub operation_ref: String,
    /// Opaque persisted-operation id ref (hash or alias, not raw text).
    pub persisted_id_ref: String,
    /// Binding state.
    pub binding_state: PersistedOperationBindingState,
    /// Contract ref the binding belongs to.
    pub contract_ref: String,
    /// Whether drift blocks any silent fallback to raw execution.
    pub drift_blocks_silent_raw_fallback: bool,
    /// Whether the binding ever falls back to raw execution silently.
    pub falls_back_to_raw_silently: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One retention-class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionClassRow {
    /// Stable retention class id.
    pub retention_id: String,
    /// Scope the class applies to.
    pub scope: RetentionScope,
    /// Retention mode.
    pub mode: RetentionMode,
    /// Whether raw bodies are excluded by default.
    pub default_excludes_bodies: bool,
    /// Whether raw headers are excluded by default.
    pub default_excludes_headers: bool,
    /// Whether full body/header capture requires explicit opt-in.
    pub opt_in_required_for_full_capture: bool,
    /// Offline or mirror behavior.
    pub offline_mirror_behavior: OfflineMirrorBehavior,
    /// Whether an offline mirror is maintained.
    pub mirror_maintained: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Reference to an upstream M5 packet this matrix builds on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpstreamRefRow {
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

/// Summary counts for an API-collection matrix qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiMatrixQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of contract rows.
    pub contract_count: usize,
    /// Number of operation-collection rows.
    pub collection_count: usize,
    /// Number of request-list rows.
    pub request_count: usize,
    /// Number of request-origin rows.
    pub origin_count: usize,
    /// Number of persisted-operation binding rows.
    pub persisted_operation_count: usize,
    /// Number of retention-class rows.
    pub retention_class_count: usize,
    /// Number of upstream reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical API-collection matrix qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiMatrixQualificationPacket {
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
    pub surfaces: Vec<ApiMatrixSurfaceQualificationRow>,
    /// Contract rows.
    pub contracts: Vec<ContractRow>,
    /// Operation-collection rows.
    pub collections: Vec<ApiCollectionRow>,
    /// Request-list rows.
    pub requests: Vec<RequestListRow>,
    /// Request-origin rows.
    pub origins: Vec<RequestOriginRow>,
    /// Persisted-operation binding rows.
    pub persisted_operations: Vec<PersistedOperationBindingRow>,
    /// Retention-class rows.
    pub retention_classes: Vec<RetentionClassRow>,
    /// Upstream reference rows.
    pub upstream_refs: Vec<UpstreamRefRow>,
    /// Summary counts.
    pub summary: ApiMatrixQualificationSummary,
}

impl ApiMatrixQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> ApiMatrixQualificationSummary {
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
        ApiMatrixQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            contract_count: self.contracts.len(),
            collection_count: self.collections.len(),
            request_count: self.requests.len(),
            origin_count: self.origins.len(),
            persisted_operation_count: self.persisted_operations.len(),
            retention_class_count: self.retention_classes.len(),
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of contracts whose freshness must narrow a stable claim
    /// (stale schema or unavailable contract).
    pub fn narrowing_contract_ids(&self) -> Vec<String> {
        self.contracts
            .iter()
            .filter(|row| row.freshness_state.narrows_claim())
            .map(|row| row.contract_id.clone())
            .collect()
    }

    /// Returns the ids of persisted-operation bindings that have drifted.
    pub fn persisted_operation_drift_ids(&self) -> Vec<String> {
        self.persisted_operations
            .iter()
            .filter(|row| {
                row.binding_state == PersistedOperationBindingState::PersistedOperationDrift
            })
            .map(|row| row.binding_id.clone())
            .collect()
    }

    /// Returns the ids of origins that changed since they were last resolved.
    pub fn changed_origin_ids(&self) -> Vec<String> {
        self.origins
            .iter()
            .filter(|row| row.drift_state == RequestOriginDriftState::OriginChanged)
            .map(|row| row.origin_id.clone())
            .collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<ApiMatrixQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != API_MATRIX_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ApiMatrixQualificationViolation::SchemaVersion {
                expected: API_MATRIX_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != API_MATRIX_QUALIFICATION_RECORD_KIND {
            violations.push(ApiMatrixQualificationViolation::RecordKind {
                expected: API_MATRIX_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces.iter().map(|row| row.surface_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::Surface,
        );
        let contract_ids = collect_ids(
            self.contracts.iter().map(|row| row.contract_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::Contract,
        );
        collect_ids(
            self.collections
                .iter()
                .map(|row| row.collection_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::Collection,
        );
        let request_ids = collect_ids(
            self.requests.iter().map(|row| row.request_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::Request,
        );
        let origin_ids = collect_ids(
            self.origins.iter().map(|row| row.origin_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::Origin,
        );
        let binding_ids = collect_ids(
            self.persisted_operations
                .iter()
                .map(|row| row.binding_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::PersistedOperation,
        );
        let retention_ids = collect_ids(
            self.retention_classes
                .iter()
                .map(|row| row.retention_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::RetentionClass,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            ApiMatrixQualificationViolationKind::UpstreamRef,
        );

        self.validate_surfaces(&mut violations);
        self.validate_contracts(&mut violations, &origin_ids, &binding_ids);
        self.validate_collections(&mut violations, &request_ids, &contract_ids, &retention_ids);
        self.validate_requests(&mut violations, &origin_ids, &binding_ids);
        self.validate_origins(&mut violations);
        self.validate_persisted_operations(&mut violations, &contract_ids);
        self.validate_retention_classes(&mut violations);
        self.validate_upstream_refs(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(ApiMatrixQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_surfaces(&self, violations: &mut Vec<ApiMatrixQualificationViolation>) {
        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(ApiMatrixQualificationViolation::StableSurfaceMissingProof {
                        surface_id: surface.surface_id.clone(),
                    });
                }
                if !surface.guards.all_visible() {
                    violations.push(ApiMatrixQualificationViolation::StableSurfaceMissingGuard {
                        surface_id: surface.surface_id.clone(),
                    });
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    ApiMatrixQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let surface_kinds: BTreeSet<_> = self.surfaces.iter().map(|row| row.surface_kind).collect();
        for required_kind in [
            ApiMatrixSurfaceKind::RequestWorkspace,
            ApiMatrixSurfaceKind::CliHeadlessOutput,
            ApiMatrixSurfaceKind::DiagnosticsSurface,
            ApiMatrixSurfaceKind::SupportExport,
            ApiMatrixSurfaceKind::CertificationScorecard,
        ] {
            if !surface_kinds.contains(&required_kind) {
                violations.push(ApiMatrixQualificationViolation::MissingSurfaceKind {
                    surface_kind: required_kind,
                });
            }
        }
    }

    fn validate_contracts(
        &self,
        violations: &mut Vec<ApiMatrixQualificationViolation>,
        origin_ids: &BTreeSet<String>,
        binding_ids: &BTreeSet<String>,
    ) {
        let contract_kinds: BTreeSet<_> =
            self.contracts.iter().map(|row| row.contract_kind).collect();
        for required_kind in [
            ContractKind::Rest,
            ContractKind::Graphql,
            ContractKind::PluginOwned,
        ] {
            if !contract_kinds.contains(&required_kind) {
                violations.push(ApiMatrixQualificationViolation::MissingContractKind {
                    contract_kind: required_kind,
                });
            }
        }

        let source_classes: BTreeSet<_> =
            self.contracts.iter().map(|row| row.source_class).collect();
        for required_source in [
            ContractSourceClass::LiveContract,
            ContractSourceClass::CachedSchema,
            ContractSourceClass::ImportedSnapshot,
            ContractSourceClass::PluginProvided,
            ContractSourceClass::ContractUnavailable,
        ] {
            if !source_classes.contains(&required_source) {
                violations.push(ApiMatrixQualificationViolation::MissingContractSource {
                    source_class: required_source,
                });
            }
        }

        let freshness_states: BTreeSet<_> = self
            .contracts
            .iter()
            .map(|row| row.freshness_state)
            .collect();
        for required_state in [
            ContractFreshnessState::LiveContract,
            ContractFreshnessState::CachedSchema,
            ContractFreshnessState::SchemaStale,
            ContractFreshnessState::ImportedSnapshot,
            ContractFreshnessState::ContractUnavailable,
        ] {
            if !freshness_states.contains(&required_state) {
                violations.push(ApiMatrixQualificationViolation::MissingFreshnessState {
                    freshness_state: required_state,
                });
            }
        }

        for row in &self.contracts {
            if row.origin_ref.is_empty()
                || !origin_ids.contains(&row.origin_ref)
                || !row.source_badge_visible
                || !row.freshness_visible
            {
                violations.push(ApiMatrixQualificationViolation::IncompleteContract {
                    contract_id: row.contract_id.clone(),
                });
            }
            // A stale or unavailable contract must be labeled and must never
            // masquerade as live truth.
            if matches!(
                row.freshness_state,
                ContractFreshnessState::SchemaStale | ContractFreshnessState::ContractUnavailable
            ) && !row.stale_labeled
            {
                violations.push(ApiMatrixQualificationViolation::StaleContractNotLabeled {
                    contract_id: row.contract_id.clone(),
                });
            }
            if row.may_masquerade_as_live {
                violations.push(ApiMatrixQualificationViolation::ContractMayMasquerade {
                    contract_id: row.contract_id.clone(),
                });
            }
            if let Some(binding_ref) = &row.persisted_operation_ref {
                if !binding_ids.contains(binding_ref) {
                    violations.push(ApiMatrixQualificationViolation::DanglingPersistedRef {
                        contract_id: row.contract_id.clone(),
                    });
                }
            }
        }
    }

    fn validate_collections(
        &self,
        violations: &mut Vec<ApiMatrixQualificationViolation>,
        request_ids: &BTreeSet<String>,
        contract_ids: &BTreeSet<String>,
        retention_ids: &BTreeSet<String>,
    ) {
        for row in &self.collections {
            let request_refs_ok = !row.request_refs.is_empty()
                && row.request_refs.iter().all(|r| request_ids.contains(r));
            let contract_refs_ok = row.contract_refs.iter().all(|r| contract_ids.contains(r));
            if !request_refs_ok
                || !contract_refs_ok
                || !retention_ids.contains(&row.retention_class_ref)
                || !row.text_first_versioned
                || !row.diffable
            {
                violations.push(ApiMatrixQualificationViolation::IncompleteCollection {
                    collection_id: row.collection_id.clone(),
                });
            }
        }
    }

    fn validate_requests(
        &self,
        violations: &mut Vec<ApiMatrixQualificationViolation>,
        origin_ids: &BTreeSet<String>,
        binding_ids: &BTreeSet<String>,
    ) {
        for row in &self.requests {
            if row.origin_ref.is_empty() || !origin_ids.contains(&row.origin_ref) {
                violations.push(ApiMatrixQualificationViolation::IncompleteRequest {
                    request_id: row.request_id.clone(),
                });
            }
            // Persisted-operation bindings and raw local text must not collapse
            // into the same truth object.
            let consistent = match row.binding_kind {
                RequestBindingKind::RawLocalText => {
                    row.raw_local_text && row.persisted_operation_ref.is_none()
                }
                RequestBindingKind::PersistedOperation => {
                    !row.raw_local_text
                        && row
                            .persisted_operation_ref
                            .as_ref()
                            .is_some_and(|r| binding_ids.contains(r))
                }
            };
            if !consistent {
                violations.push(ApiMatrixQualificationViolation::RequestBindingAmbiguous {
                    request_id: row.request_id.clone(),
                });
            }
        }
    }

    fn validate_origins(&self, violations: &mut Vec<ApiMatrixQualificationViolation>) {
        let origin_kinds: BTreeSet<_> = self.origins.iter().map(|row| row.origin_kind).collect();
        for required_kind in [
            RequestOriginKind::LocalHost,
            RequestOriginKind::Remote,
            RequestOriginKind::Container,
            RequestOriginKind::Managed,
            RequestOriginKind::BrowserCompanion,
        ] {
            if !origin_kinds.contains(&required_kind) {
                violations.push(ApiMatrixQualificationViolation::MissingOriginKind {
                    origin_kind: required_kind,
                });
            }
        }

        for row in &self.origins {
            if !row.explicit_naming || !row.retarget_requires_ack || row.target_label.is_empty() {
                violations.push(ApiMatrixQualificationViolation::IncompleteOrigin {
                    origin_id: row.origin_id.clone(),
                });
            }
            // Browser-companion and managed origins must never inherit local trust.
            if row.origin_kind.must_isolate_local_trust() && row.inherits_local_trust {
                violations.push(ApiMatrixQualificationViolation::OriginInheritsLocalTrust {
                    origin_id: row.origin_id.clone(),
                });
            }
        }
    }

    fn validate_persisted_operations(
        &self,
        violations: &mut Vec<ApiMatrixQualificationViolation>,
        contract_ids: &BTreeSet<String>,
    ) {
        let states: BTreeSet<_> = self
            .persisted_operations
            .iter()
            .map(|row| row.binding_state)
            .collect();
        for required_state in [
            PersistedOperationBindingState::NotPersisted,
            PersistedOperationBindingState::BoundCurrent,
            PersistedOperationBindingState::PersistedOperationDrift,
        ] {
            if !states.contains(&required_state) {
                violations.push(ApiMatrixQualificationViolation::MissingPersistedState {
                    binding_state: required_state,
                });
            }
        }

        for row in &self.persisted_operations {
            let id_required = row.binding_state != PersistedOperationBindingState::NotPersisted;
            if (id_required && row.persisted_id_ref.is_empty())
                || !contract_ids.contains(&row.contract_ref)
            {
                violations.push(
                    ApiMatrixQualificationViolation::IncompletePersistedOperation {
                        binding_id: row.binding_id.clone(),
                    },
                );
            }
            // Drift must never silently fall back to raw request execution.
            if row.falls_back_to_raw_silently {
                violations.push(
                    ApiMatrixQualificationViolation::PersistedOperationSilentRawFallback {
                        binding_id: row.binding_id.clone(),
                    },
                );
            }
            if row.binding_state == PersistedOperationBindingState::PersistedOperationDrift
                && !row.drift_blocks_silent_raw_fallback
            {
                violations.push(
                    ApiMatrixQualificationViolation::PersistedOperationSilentRawFallback {
                        binding_id: row.binding_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_retention_classes(&self, violations: &mut Vec<ApiMatrixQualificationViolation>) {
        let scopes: BTreeSet<_> = self.retention_classes.iter().map(|row| row.scope).collect();
        for required_scope in [RetentionScope::Collection, RetentionScope::History] {
            if !scopes.contains(&required_scope) {
                violations.push(ApiMatrixQualificationViolation::MissingRetentionScope {
                    scope: required_scope,
                });
            }
        }

        for row in &self.retention_classes {
            // Request history must never retain raw bodies or headers by default.
            if row.scope == RetentionScope::History
                && (!row.default_excludes_bodies
                    || !row.default_excludes_headers
                    || !row.opt_in_required_for_full_capture)
            {
                violations.push(ApiMatrixQualificationViolation::UnsafeHistoryRetention {
                    retention_id: row.retention_id.clone(),
                });
            }
        }
    }

    fn validate_upstream_refs(&self, violations: &mut Vec<ApiMatrixQualificationViolation>) {
        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(ApiMatrixQualificationViolation::IncompleteUpstreamRef {
                    ref_id: row.ref_id.clone(),
                });
            }
        }
    }
}

/// Loads the checked-in API-collection matrix qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_api_matrix_qualification() -> Result<ApiMatrixQualificationPacket, serde_json::Error>
{
    serde_json::from_str(API_MATRIX_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiMatrixQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Contract rows.
    Contract,
    /// Operation-collection rows.
    Collection,
    /// Request-list rows.
    Request,
    /// Request-origin rows.
    Origin,
    /// Persisted-operation binding rows.
    PersistedOperation,
    /// Retention-class rows.
    RetentionClass,
    /// Upstream reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<ApiMatrixQualificationViolation>,
    kind: ApiMatrixQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(ApiMatrixQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for API-collection matrix qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiMatrixQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: ApiMatrixQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required consumer surface kind is missing.
    MissingSurfaceKind { surface_kind: ApiMatrixSurfaceKind },
    /// Required contract kind is missing.
    MissingContractKind { contract_kind: ContractKind },
    /// Required contract source class is missing.
    MissingContractSource { source_class: ContractSourceClass },
    /// Required freshness state is missing.
    MissingFreshnessState {
        freshness_state: ContractFreshnessState,
    },
    /// Contract row does not project source/freshness/origin truth everywhere.
    IncompleteContract { contract_id: String },
    /// Stale or unavailable contract is not explicitly labeled.
    StaleContractNotLabeled { contract_id: String },
    /// Contract may masquerade stale or imported schema as live truth.
    ContractMayMasquerade { contract_id: String },
    /// Contract references a persisted operation that does not exist.
    DanglingPersistedRef { contract_id: String },
    /// Collection row does not project text-first, diffable, referenced truth.
    IncompleteCollection { collection_id: String },
    /// Request row does not resolve an origin.
    IncompleteRequest { request_id: String },
    /// Request collapses persisted-operation binding and raw text together.
    RequestBindingAmbiguous { request_id: String },
    /// Required origin kind is missing.
    MissingOriginKind { origin_kind: RequestOriginKind },
    /// Origin row does not keep explicit naming and retarget acknowledgement.
    IncompleteOrigin { origin_id: String },
    /// Browser-companion or managed origin inherits desktop-local trust.
    OriginInheritsLocalTrust { origin_id: String },
    /// Required persisted-operation binding state is missing.
    MissingPersistedState {
        binding_state: PersistedOperationBindingState,
    },
    /// Persisted-operation binding lacks id or contract truth.
    IncompletePersistedOperation { binding_id: String },
    /// Persisted-operation or schema drift may fall back to raw execution silently.
    PersistedOperationSilentRawFallback { binding_id: String },
    /// Required retention scope is missing.
    MissingRetentionScope { scope: RetentionScope },
    /// History retention retains raw bodies or headers by default.
    UnsafeHistoryRetention { retention_id: String },
    /// Upstream reference is incomplete.
    IncompleteUpstreamRef { ref_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for ApiMatrixQualificationViolation {
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
            Self::MissingContractKind { contract_kind } => {
                write!(f, "contract kind {contract_kind:?} is not covered")
            }
            Self::MissingContractSource { source_class } => {
                write!(f, "contract source class {source_class:?} is not covered")
            }
            Self::MissingFreshnessState { freshness_state } => {
                write!(f, "freshness state {freshness_state:?} is not covered")
            }
            Self::IncompleteContract { contract_id } => {
                write!(
                    f,
                    "{contract_id} does not project contract truth everywhere"
                )
            }
            Self::StaleContractNotLabeled { contract_id } => {
                write!(f, "{contract_id} is stale or unavailable without a label")
            }
            Self::ContractMayMasquerade { contract_id } => {
                write!(
                    f,
                    "{contract_id} may masquerade non-live schema as live truth"
                )
            }
            Self::DanglingPersistedRef { contract_id } => {
                write!(f, "{contract_id} references an unknown persisted operation")
            }
            Self::IncompleteCollection { collection_id } => {
                write!(
                    f,
                    "{collection_id} does not project collection truth everywhere"
                )
            }
            Self::IncompleteRequest { request_id } => {
                write!(f, "{request_id} does not resolve a request origin")
            }
            Self::RequestBindingAmbiguous { request_id } => {
                write!(
                    f,
                    "{request_id} collapses persisted-operation binding and raw text"
                )
            }
            Self::MissingOriginKind { origin_kind } => {
                write!(f, "request origin kind {origin_kind:?} is not covered")
            }
            Self::IncompleteOrigin { origin_id } => {
                write!(
                    f,
                    "{origin_id} does not keep explicit, acknowledged origin truth"
                )
            }
            Self::OriginInheritsLocalTrust { origin_id } => {
                write!(
                    f,
                    "{origin_id} inherits desktop-local trust it must not have"
                )
            }
            Self::MissingPersistedState { binding_state } => {
                write!(
                    f,
                    "persisted-operation binding state {binding_state:?} is not covered"
                )
            }
            Self::IncompletePersistedOperation { binding_id } => {
                write!(f, "{binding_id} lacks persisted-id or contract truth")
            }
            Self::PersistedOperationSilentRawFallback { binding_id } => {
                write!(f, "{binding_id} may fall back to raw execution silently")
            }
            Self::MissingRetentionScope { scope } => {
                write!(f, "retention scope {scope:?} is not covered")
            }
            Self::UnsafeHistoryRetention { retention_id } => {
                write!(f, "{retention_id} retains raw bodies or headers by default")
            }
            Self::IncompleteUpstreamRef { ref_id } => {
                write!(
                    f,
                    "{ref_id} does not project upstream reference truth everywhere"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for ApiMatrixQualificationViolation {}
