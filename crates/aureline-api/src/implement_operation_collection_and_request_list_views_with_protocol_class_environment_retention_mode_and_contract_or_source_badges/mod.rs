//! Operation-collection and request-list view qualification records.
//!
//! This module owns the typed records that render the operation-collection tree
//! and request-list surfaces on top of the frozen API-collection matrix. Each
//! collection-view and request-list-view row keeps protocol class, environment
//! identity, contract/source badge, schema freshness, last-run state, retention
//! mode, provenance, and open-detail/inspect/export actions inspectable so large
//! API workspaces stay legible, versionable, and reviewable instead of
//! degenerating into ad hoc file trees.
//!
//! These records reuse the canonical matrix vocabulary
//! ([`ContractSourceClass`], [`ContractFreshnessState`], [`RetentionMode`]) and
//! the composer export-redaction vocabulary ([`ExportRedactionClass`]) rather
//! than minting local synonyms; the views are a real consumer of the
//! [`freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix`](crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix)
//! truth and reference it as a verified upstream packet.
//!
//! Raw endpoint URLs, raw secrets, raw request bodies, raw headers, and raw
//! schema payloads do not belong in these records. Rows carry stable IDs, closed
//! posture vocabularies, opaque refs, and reviewable summaries. Request files
//! stay text-first and versionable; saved views carry no opaque binary state
//! that would block repo review; schema freshness and persisted-operation drift
//! never hide behind a green last-run state; environment and contract identity
//! are never reduced to a friendly name alone; and shared or managed views never
//! inherit desktop-local trust.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix::{
    ContractFreshnessState, ContractSourceClass, RetentionMode,
    API_MATRIX_QUALIFICATION_RECORD_KIND,
};
use crate::implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export::ExportRedactionClass;

/// Supported schema version for request-views qualification packets.
pub const REQUEST_VIEWS_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RequestViewsQualificationPacket`].
pub const REQUEST_VIEWS_QUALIFICATION_RECORD_KIND: &str =
    "implement_operation_collection_and_request_list_views_with_protocol_class_environment_retention_mode_and_contract_or_source_badges";

/// Repo-relative path to the checked-in request-views packet.
pub const REQUEST_VIEWS_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.json";

/// Embedded checked-in packet JSON.
pub const REQUEST_VIEWS_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.json"
));

/// Qualification label shown on promoted view surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestViewsQualificationLabel {
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

impl RequestViewsQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// View-consumer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestViewsSurfaceKind {
    /// Request-workspace collection tree and request list.
    RequestWorkspaceTree,
    /// Keyboard-first command-palette quick-open list.
    CommandPaletteList,
    /// CLI or headless request-list rendering.
    CliHeadlessList,
    /// Support-export view carrying collection and request-list truth.
    SupportExportView,
    /// Help/About surface describing the view contract.
    HelpAboutView,
}

/// Wire-protocol class shown as the request-list protocol badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolClass {
    /// REST over HTTP.
    Rest,
    /// GraphQL operation.
    Graphql,
    /// gRPC method (typically provider- or plugin-linked).
    Grpc,
    /// WebSocket session.
    Websocket,
}

/// Named-environment posture shown in the environment column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentClass {
    /// Local desktop environment.
    Local,
    /// Development environment.
    Development,
    /// Staging or pre-production environment.
    Staging,
    /// Production environment.
    Production,
    /// Managed or shared cloud-hosted environment.
    Managed,
}

impl EnvironmentClass {
    /// Returns true when the environment must never inherit desktop-local trust
    /// or naming assumptions.
    pub const fn must_isolate_local_trust(self) -> bool {
        matches!(self, Self::Managed)
    }
}

/// Last-run state shown for a request-list row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LastRunState {
    /// Request has never been run from this workspace.
    NeverRun,
    /// Last run succeeded against a current contract.
    Succeeded,
    /// Last run failed.
    Failed,
    /// Last run is blocked pending contract or drift review.
    BlockedPendingReview,
    /// Prior result is stale and an explicit re-send is required.
    StaleNeedsResend,
}

impl LastRunState {
    /// Returns true when the row shows a clean pass that would otherwise mask
    /// contract risk behind a green send button.
    pub const fn is_clean_pass(self) -> bool {
        matches!(self, Self::Succeeded)
    }
}

/// Provenance class that distinguishes how a request-list row is sourced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestProvenanceClass {
    /// Local-only history captured on this desktop.
    LocalOnlyHistory,
    /// Imported snapshot loaded from a file or workspace artifact.
    ImportedSnapshot,
    /// Provider-linked contract row owned by a provider or plugin.
    ProviderLinkedContract,
    /// Managed or shared artifact published into the workspace.
    ManagedSharedArtifact,
}

impl RequestProvenanceClass {
    /// Returns true when the provenance is shared or managed and must never
    /// inherit desktop-local trust.
    pub const fn must_isolate_local_trust(self) -> bool {
        matches!(self, Self::ManagedSharedArtifact)
    }
}

/// Privacy scope of a saved view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewVisibility {
    /// View is private and local to this desktop.
    PrivateLocal,
    /// View is shared across the workspace.
    WorkspaceShared,
}

impl SavedViewVisibility {
    /// Returns true when the saved view is shared and must never inherit
    /// desktop-local trust.
    pub const fn must_isolate_local_trust(self) -> bool {
        matches!(self, Self::WorkspaceShared)
    }
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestViewsQualificationProof {
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

/// Boolean guard set that keeps stable view surfaces honest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestViewsSurfaceGuardSet {
    /// Protocol class is visible on the request list.
    pub protocol_class_visible: bool,
    /// Environment identity is visible.
    pub environment_visible: bool,
    /// Contract-source and freshness badge is visible.
    pub contract_source_badge_visible: bool,
    /// Last-run state is visible.
    pub last_run_state_visible: bool,
    /// Retention mode is visible.
    pub retention_mode_visible: bool,
    /// Request provenance is visible.
    pub provenance_visible: bool,
    /// The surface is keyboard navigable.
    pub keyboard_navigable: bool,
    /// Rows carry stable IDs.
    pub stable_ids: bool,
    /// Open-detail, inspect, and export actions are available.
    pub detail_inspect_export_actions: bool,
    /// Environment and contract identity are never hidden behind friendly names alone.
    pub identity_not_friendly_name_only: bool,
    /// Schema freshness or persisted-operation drift never hides behind a green last-run state.
    pub drift_not_hidden_by_green_send: bool,
    /// Shared or managed views never inherit desktop-local trust.
    pub shared_view_trust_isolated: bool,
    /// Collections and saved views stay text-first and diffable.
    pub text_first_diffable: bool,
}

impl RequestViewsSurfaceGuardSet {
    /// Returns true when every required guard is present.
    pub const fn all_visible(&self) -> bool {
        self.protocol_class_visible
            && self.environment_visible
            && self.contract_source_badge_visible
            && self.last_run_state_visible
            && self.retention_mode_visible
            && self.provenance_visible
            && self.keyboard_navigable
            && self.stable_ids
            && self.detail_inspect_export_actions
            && self.identity_not_friendly_name_only
            && self.drift_not_hidden_by_green_send
            && self.shared_view_trust_isolated
            && self.text_first_diffable
    }
}

/// One governed view-consumer surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestViewsSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: RequestViewsSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: RequestViewsQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: RequestViewsQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<RequestViewsQualificationProof>,
    /// Visible guard set.
    pub guards: RequestViewsSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One operation-collection tree-view row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectionViewRow {
    /// Stable collection-view id.
    pub collection_view_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Matrix collection ref this view renders.
    pub matrix_collection_ref: String,
    /// Request-list-view member refs.
    pub request_view_refs: Vec<String>,
    /// Protocol classes present in the collection.
    pub protocol_classes: Vec<ProtocolClass>,
    /// Environment-view refs in scope for this collection.
    pub environment_refs: Vec<String>,
    /// Whether the tree is keyboard navigable.
    pub keyboard_navigable: bool,
    /// Whether the row carries a stable id.
    pub stable_id: bool,
    /// Whether open-detail is available.
    pub open_detail_available: bool,
    /// Whether export is available.
    pub export_available: bool,
    /// Export redaction posture for the collection export action.
    pub export_redaction: ExportRedactionClass,
    /// Whether request definitions stay text-first and versionable.
    pub text_first_versioned: bool,
    /// Whether the collection is diffable.
    pub diffable: bool,
    /// Whether the row hides protocol or environment identity.
    pub hides_protocol_or_environment_identity: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One request-list-view row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestListViewRow {
    /// Stable request-view id.
    pub request_view_id: String,
    /// Owning collection-view ref.
    pub collection_view_ref: String,
    /// Matrix request ref this view renders.
    pub matrix_request_ref: String,
    /// Protocol class badge.
    pub protocol_class: ProtocolClass,
    /// Environment-view ref.
    pub environment_ref: String,
    /// Provenance class.
    pub provenance_class: RequestProvenanceClass,
    /// Contract-source class shown on the badge.
    pub contract_source_class: ContractSourceClass,
    /// Schema-freshness state shown on the badge.
    pub contract_freshness_state: ContractFreshnessState,
    /// Last-run state.
    pub last_run_state: LastRunState,
    /// Retention mode shown for the row.
    pub retention_mode: RetentionMode,
    /// Whether the contract-source and freshness badge is visible.
    pub contract_source_badge_visible: bool,
    /// Whether the last-run state is visible.
    pub last_run_state_visible: bool,
    /// Whether the retention mode is visible.
    pub retention_mode_visible: bool,
    /// Whether inspect is available.
    pub inspect_available: bool,
    /// Whether export is available.
    pub export_available: bool,
    /// Export redaction posture for the row export action.
    pub export_redaction: ExportRedactionClass,
    /// Whether schema or persisted-operation drift is hidden behind a green send.
    pub drift_hidden_by_green_send: bool,
    /// Whether environment or contract identity is reduced to a friendly name alone.
    pub friendly_name_only_identity: bool,
    /// Whether the row carries a stable id.
    pub stable_id: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One environment-column row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentViewRow {
    /// Stable environment-view id.
    pub environment_view_id: String,
    /// Friendly display name.
    pub friendly_name: String,
    /// Environment class shown alongside the friendly name.
    pub environment_class: EnvironmentClass,
    /// Opaque, non-secret resolved-target label.
    pub resolved_target_label: String,
    /// Matrix origin ref this environment resolves against, if any.
    pub matrix_origin_ref: Option<String>,
    /// Whether the environment inherits desktop-local trust.
    pub inherits_local_trust: bool,
    /// Whether explicit environment identity is visible beyond the friendly name.
    pub explicit_identity_visible: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One saved-view row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedViewRow {
    /// Stable saved-view id.
    pub saved_view_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Privacy scope.
    pub visibility: SavedViewVisibility,
    /// Collection-view refs included in the saved view.
    pub collection_view_refs: Vec<String>,
    /// Reviewable, text-first filter summary (never opaque binary state).
    pub filter_summary: String,
    /// Whether the saved view stores opaque binary state that blocks review.
    pub opaque_binary_state: bool,
    /// Whether the saved view inherits desktop-local trust.
    pub inherits_local_trust: bool,
    /// Whether the saved view is diffable.
    pub diffable: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Reference to an upstream M5 packet these views consume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestViewsUpstreamRefRow {
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

/// Summary counts for a request-views qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestViewsQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of collection-view rows.
    pub collection_view_count: usize,
    /// Number of request-list-view rows.
    pub request_view_count: usize,
    /// Number of environment-view rows.
    pub environment_view_count: usize,
    /// Number of saved-view rows.
    pub saved_view_count: usize,
    /// Number of upstream reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical request-views qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestViewsQualificationPacket {
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
    pub surfaces: Vec<RequestViewsSurfaceQualificationRow>,
    /// Operation-collection view rows.
    pub collection_views: Vec<CollectionViewRow>,
    /// Request-list view rows.
    pub request_views: Vec<RequestListViewRow>,
    /// Environment-view rows.
    pub environment_views: Vec<EnvironmentViewRow>,
    /// Saved-view rows.
    pub saved_views: Vec<SavedViewRow>,
    /// Upstream reference rows.
    pub upstream_refs: Vec<RequestViewsUpstreamRefRow>,
    /// Summary counts.
    pub summary: RequestViewsQualificationSummary,
}

impl RequestViewsQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> RequestViewsQualificationSummary {
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
        RequestViewsQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            collection_view_count: self.collection_views.len(),
            request_view_count: self.request_views.len(),
            environment_view_count: self.environment_views.len(),
            saved_view_count: self.saved_views.len(),
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of request-list rows whose provenance is a provider-linked
    /// contract.
    pub fn provider_linked_request_ids(&self) -> Vec<String> {
        self.request_views
            .iter()
            .filter(|row| row.provenance_class == RequestProvenanceClass::ProviderLinkedContract)
            .map(|row| row.request_view_id.clone())
            .collect()
    }

    /// Returns the ids of request-list rows that are blocked or stale pending a
    /// contract or drift review, so consumers can narrow instead of trusting a
    /// green send.
    pub fn drift_blocked_request_ids(&self) -> Vec<String> {
        self.request_views
            .iter()
            .filter(|row| {
                matches!(
                    row.last_run_state,
                    LastRunState::BlockedPendingReview | LastRunState::StaleNeedsResend
                )
            })
            .map(|row| row.request_view_id.clone())
            .collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<RequestViewsQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != REQUEST_VIEWS_QUALIFICATION_SCHEMA_VERSION {
            violations.push(RequestViewsQualificationViolation::SchemaVersion {
                expected: REQUEST_VIEWS_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REQUEST_VIEWS_QUALIFICATION_RECORD_KIND {
            violations.push(RequestViewsQualificationViolation::RecordKind {
                expected: REQUEST_VIEWS_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces.iter().map(|row| row.surface_id.as_str()),
            &mut violations,
            RequestViewsViolationKind::Surface,
        );
        let collection_ids = collect_ids(
            self.collection_views
                .iter()
                .map(|row| row.collection_view_id.as_str()),
            &mut violations,
            RequestViewsViolationKind::CollectionView,
        );
        let request_ids = collect_ids(
            self.request_views
                .iter()
                .map(|row| row.request_view_id.as_str()),
            &mut violations,
            RequestViewsViolationKind::RequestView,
        );
        let environment_ids = collect_ids(
            self.environment_views
                .iter()
                .map(|row| row.environment_view_id.as_str()),
            &mut violations,
            RequestViewsViolationKind::EnvironmentView,
        );
        collect_ids(
            self.saved_views
                .iter()
                .map(|row| row.saved_view_id.as_str()),
            &mut violations,
            RequestViewsViolationKind::SavedView,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            RequestViewsViolationKind::UpstreamRef,
        );

        self.validate_surfaces(&mut violations);
        self.validate_collection_views(&mut violations, &request_ids, &environment_ids);
        self.validate_request_views(&mut violations, &collection_ids, &environment_ids);
        self.validate_environment_views(&mut violations);
        self.validate_saved_views(&mut violations, &collection_ids);
        self.validate_upstream_refs(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(RequestViewsQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_surfaces(&self, violations: &mut Vec<RequestViewsQualificationViolation>) {
        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        RequestViewsQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        RequestViewsQualificationViolation::StableSurfaceMissingGuard {
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
                    RequestViewsQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let surface_kinds: BTreeSet<_> = self.surfaces.iter().map(|row| row.surface_kind).collect();
        for required_kind in [
            RequestViewsSurfaceKind::RequestWorkspaceTree,
            RequestViewsSurfaceKind::CommandPaletteList,
            RequestViewsSurfaceKind::CliHeadlessList,
            RequestViewsSurfaceKind::SupportExportView,
            RequestViewsSurfaceKind::HelpAboutView,
        ] {
            if !surface_kinds.contains(&required_kind) {
                violations.push(RequestViewsQualificationViolation::MissingSurfaceKind {
                    surface_kind: required_kind,
                });
            }
        }
    }

    fn validate_collection_views(
        &self,
        violations: &mut Vec<RequestViewsQualificationViolation>,
        request_ids: &BTreeSet<String>,
        environment_ids: &BTreeSet<String>,
    ) {
        for row in &self.collection_views {
            let request_refs_ok = !row.request_view_refs.is_empty()
                && row
                    .request_view_refs
                    .iter()
                    .all(|r| request_ids.contains(r));
            let environment_refs_ok = !row.environment_refs.is_empty()
                && row
                    .environment_refs
                    .iter()
                    .all(|r| environment_ids.contains(r));
            if row.matrix_collection_ref.is_empty()
                || !request_refs_ok
                || !environment_refs_ok
                || row.protocol_classes.is_empty()
                || !row.keyboard_navigable
                || !row.stable_id
                || !row.open_detail_available
                || !row.text_first_versioned
                || !row.diffable
            {
                violations.push(
                    RequestViewsQualificationViolation::IncompleteCollectionView {
                        collection_view_id: row.collection_view_id.clone(),
                    },
                );
            }
            // Collections must never hide protocol or environment identity.
            if row.hides_protocol_or_environment_identity {
                violations.push(
                    RequestViewsQualificationViolation::CollectionHidesIdentity {
                        collection_view_id: row.collection_view_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_request_views(
        &self,
        violations: &mut Vec<RequestViewsQualificationViolation>,
        collection_ids: &BTreeSet<String>,
        environment_ids: &BTreeSet<String>,
    ) {
        for row in &self.request_views {
            if !collection_ids.contains(&row.collection_view_ref)
                || !environment_ids.contains(&row.environment_ref)
                || row.matrix_request_ref.is_empty()
                || !row.contract_source_badge_visible
                || !row.last_run_state_visible
                || !row.retention_mode_visible
                || !row.stable_id
            {
                violations.push(RequestViewsQualificationViolation::IncompleteRequestView {
                    request_view_id: row.request_view_id.clone(),
                });
            }
            // Schema freshness or persisted-operation drift must never hide
            // behind a green last-run state.
            if row.drift_hidden_by_green_send {
                violations.push(RequestViewsQualificationViolation::DriftHiddenByGreenSend {
                    request_view_id: row.request_view_id.clone(),
                });
            }
            if row.contract_freshness_state.narrows_claim() && row.last_run_state.is_clean_pass() {
                violations.push(RequestViewsQualificationViolation::DriftHiddenByGreenSend {
                    request_view_id: row.request_view_id.clone(),
                });
            }
            // Environment and contract identity must never be a friendly name alone.
            if row.friendly_name_only_identity {
                violations.push(
                    RequestViewsQualificationViolation::IdentityFriendlyNameOnly {
                        request_view_id: row.request_view_id.clone(),
                    },
                );
            }
            // Shared/managed provenance must never inherit desktop-local trust;
            // it must resolve to a managed environment.
            if row.provenance_class.must_isolate_local_trust() {
                let env_is_managed = self
                    .environment_views
                    .iter()
                    .find(|env| env.environment_view_id == row.environment_ref)
                    .is_some_and(|env| {
                        env.environment_class.must_isolate_local_trust()
                            && !env.inherits_local_trust
                    });
                if !env_is_managed {
                    violations.push(
                        RequestViewsQualificationViolation::SharedRequestInheritsLocalTrust {
                            request_view_id: row.request_view_id.clone(),
                        },
                    );
                }
            }
        }

        let provenance_classes: BTreeSet<_> = self
            .request_views
            .iter()
            .map(|row| row.provenance_class)
            .collect();
        for required_class in [
            RequestProvenanceClass::LocalOnlyHistory,
            RequestProvenanceClass::ImportedSnapshot,
            RequestProvenanceClass::ProviderLinkedContract,
            RequestProvenanceClass::ManagedSharedArtifact,
        ] {
            if !provenance_classes.contains(&required_class) {
                violations.push(RequestViewsQualificationViolation::MissingProvenanceClass {
                    provenance_class: required_class,
                });
            }
        }

        let protocol_classes: BTreeSet<_> = self
            .request_views
            .iter()
            .map(|row| row.protocol_class)
            .collect();
        // REST and GraphQL are the protocols M5 claims; they must be covered.
        for required_protocol in [ProtocolClass::Rest, ProtocolClass::Graphql] {
            if !protocol_classes.contains(&required_protocol) {
                violations.push(RequestViewsQualificationViolation::MissingProtocolClass {
                    protocol_class: required_protocol,
                });
            }
        }
    }

    fn validate_environment_views(&self, violations: &mut Vec<RequestViewsQualificationViolation>) {
        let environment_classes: BTreeSet<_> = self
            .environment_views
            .iter()
            .map(|row| row.environment_class)
            .collect();
        for required_class in [
            EnvironmentClass::Local,
            EnvironmentClass::Development,
            EnvironmentClass::Staging,
            EnvironmentClass::Production,
            EnvironmentClass::Managed,
        ] {
            if !environment_classes.contains(&required_class) {
                violations.push(
                    RequestViewsQualificationViolation::MissingEnvironmentClass {
                        environment_class: required_class,
                    },
                );
            }
        }

        for row in &self.environment_views {
            if row.friendly_name.is_empty()
                || row.resolved_target_label.is_empty()
                || !row.explicit_identity_visible
            {
                violations.push(
                    RequestViewsQualificationViolation::IncompleteEnvironmentView {
                        environment_view_id: row.environment_view_id.clone(),
                    },
                );
            }
            // Managed environments must never inherit desktop-local trust.
            if row.environment_class.must_isolate_local_trust() && row.inherits_local_trust {
                violations.push(
                    RequestViewsQualificationViolation::EnvironmentInheritsLocalTrust {
                        environment_view_id: row.environment_view_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_saved_views(
        &self,
        violations: &mut Vec<RequestViewsQualificationViolation>,
        collection_ids: &BTreeSet<String>,
    ) {
        let visibilities: BTreeSet<_> = self.saved_views.iter().map(|row| row.visibility).collect();
        for required_visibility in [
            SavedViewVisibility::PrivateLocal,
            SavedViewVisibility::WorkspaceShared,
        ] {
            if !visibilities.contains(&required_visibility) {
                violations.push(
                    RequestViewsQualificationViolation::MissingSavedViewVisibility {
                        visibility: required_visibility,
                    },
                );
            }
        }

        for row in &self.saved_views {
            let refs_ok = !row.collection_view_refs.is_empty()
                && row
                    .collection_view_refs
                    .iter()
                    .all(|r| collection_ids.contains(r));
            if !refs_ok || row.filter_summary.is_empty() || !row.diffable {
                violations.push(RequestViewsQualificationViolation::IncompleteSavedView {
                    saved_view_id: row.saved_view_id.clone(),
                });
            }
            // Saved views must never store opaque binary state that blocks review.
            if row.opaque_binary_state {
                violations.push(
                    RequestViewsQualificationViolation::SavedViewOpaqueBinaryState {
                        saved_view_id: row.saved_view_id.clone(),
                    },
                );
            }
            // Shared views must never inherit desktop-local trust.
            if row.visibility.must_isolate_local_trust() && row.inherits_local_trust {
                violations.push(
                    RequestViewsQualificationViolation::SharedViewInheritsLocalTrust {
                        saved_view_id: row.saved_view_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_upstream_refs(&self, violations: &mut Vec<RequestViewsQualificationViolation>) {
        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(RequestViewsQualificationViolation::IncompleteUpstreamRef {
                    ref_id: row.ref_id.clone(),
                });
            }
        }
        // The views must consume the frozen API-collection matrix as a verified
        // upstream packet.
        let consumes_matrix = self.upstream_refs.iter().any(|row| {
            row.upstream_record_kind == API_MATRIX_QUALIFICATION_RECORD_KIND
                && row.integration_verified
        });
        if !consumes_matrix {
            violations.push(RequestViewsQualificationViolation::MatrixUpstreamNotIntegrated);
        }
    }
}

/// Loads the checked-in request-views qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_request_views_qualification(
) -> Result<RequestViewsQualificationPacket, serde_json::Error> {
    serde_json::from_str(REQUEST_VIEWS_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestViewsViolationKind {
    /// Surface rows.
    Surface,
    /// Operation-collection view rows.
    CollectionView,
    /// Request-list view rows.
    RequestView,
    /// Environment-view rows.
    EnvironmentView,
    /// Saved-view rows.
    SavedView,
    /// Upstream reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<RequestViewsQualificationViolation>,
    kind: RequestViewsViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(RequestViewsQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for request-views qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestViewsQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: RequestViewsViolationKind,
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
        surface_kind: RequestViewsSurfaceKind,
    },
    /// Collection-view row does not project text-first, navigable, referenced truth.
    IncompleteCollectionView { collection_view_id: String },
    /// Collection-view row hides protocol or environment identity.
    CollectionHidesIdentity { collection_view_id: String },
    /// Request-view row does not resolve its references or badges.
    IncompleteRequestView { request_view_id: String },
    /// Request-view row hides schema or persisted-operation drift behind a green send.
    DriftHiddenByGreenSend { request_view_id: String },
    /// Request-view row reduces environment or contract identity to a friendly name alone.
    IdentityFriendlyNameOnly { request_view_id: String },
    /// Shared or managed request-view row inherits desktop-local trust.
    SharedRequestInheritsLocalTrust { request_view_id: String },
    /// Required provenance class is missing.
    MissingProvenanceClass {
        provenance_class: RequestProvenanceClass,
    },
    /// Required protocol class is missing.
    MissingProtocolClass { protocol_class: ProtocolClass },
    /// Environment-view row does not project explicit, named identity.
    IncompleteEnvironmentView { environment_view_id: String },
    /// Managed environment inherits desktop-local trust.
    EnvironmentInheritsLocalTrust { environment_view_id: String },
    /// Required environment class is missing.
    MissingEnvironmentClass { environment_class: EnvironmentClass },
    /// Saved-view row does not project diffable, referenced truth.
    IncompleteSavedView { saved_view_id: String },
    /// Saved-view row stores opaque binary state that blocks review.
    SavedViewOpaqueBinaryState { saved_view_id: String },
    /// Shared saved-view row inherits desktop-local trust.
    SharedViewInheritsLocalTrust { saved_view_id: String },
    /// Required saved-view visibility is missing.
    MissingSavedViewVisibility { visibility: SavedViewVisibility },
    /// Upstream reference is incomplete.
    IncompleteUpstreamRef { ref_id: String },
    /// The views do not consume the API-collection matrix as a verified upstream packet.
    MatrixUpstreamNotIntegrated,
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for RequestViewsQualificationViolation {
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
            Self::IncompleteCollectionView { collection_view_id } => {
                write!(
                    f,
                    "{collection_view_id} does not project collection-view truth everywhere"
                )
            }
            Self::CollectionHidesIdentity { collection_view_id } => {
                write!(
                    f,
                    "{collection_view_id} hides protocol or environment identity"
                )
            }
            Self::IncompleteRequestView { request_view_id } => {
                write!(
                    f,
                    "{request_view_id} does not resolve request-view references or badges"
                )
            }
            Self::DriftHiddenByGreenSend { request_view_id } => {
                write!(
                    f,
                    "{request_view_id} hides schema or persisted-operation drift behind a green send"
                )
            }
            Self::IdentityFriendlyNameOnly { request_view_id } => {
                write!(
                    f,
                    "{request_view_id} reduces environment or contract identity to a friendly name alone"
                )
            }
            Self::SharedRequestInheritsLocalTrust { request_view_id } => {
                write!(
                    f,
                    "{request_view_id} is shared or managed but inherits desktop-local trust"
                )
            }
            Self::MissingProvenanceClass { provenance_class } => {
                write!(f, "provenance class {provenance_class:?} is not covered")
            }
            Self::MissingProtocolClass { protocol_class } => {
                write!(f, "protocol class {protocol_class:?} is not covered")
            }
            Self::IncompleteEnvironmentView {
                environment_view_id,
            } => {
                write!(
                    f,
                    "{environment_view_id} does not project explicit, named environment identity"
                )
            }
            Self::EnvironmentInheritsLocalTrust {
                environment_view_id,
            } => {
                write!(
                    f,
                    "{environment_view_id} is managed but inherits desktop-local trust"
                )
            }
            Self::MissingEnvironmentClass { environment_class } => {
                write!(f, "environment class {environment_class:?} is not covered")
            }
            Self::IncompleteSavedView { saved_view_id } => {
                write!(
                    f,
                    "{saved_view_id} does not project diffable, referenced saved-view truth"
                )
            }
            Self::SavedViewOpaqueBinaryState { saved_view_id } => {
                write!(f, "{saved_view_id} stores opaque binary state")
            }
            Self::SharedViewInheritsLocalTrust { saved_view_id } => {
                write!(
                    f,
                    "{saved_view_id} is shared but inherits desktop-local trust"
                )
            }
            Self::MissingSavedViewVisibility { visibility } => {
                write!(f, "saved-view visibility {visibility:?} is not covered")
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
                    "views do not consume the API-collection matrix as a verified upstream packet"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for RequestViewsQualificationViolation {}
