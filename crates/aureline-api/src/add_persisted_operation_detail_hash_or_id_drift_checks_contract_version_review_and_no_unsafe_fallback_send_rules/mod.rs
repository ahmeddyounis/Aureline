//! Persisted-operation detail, drift-check, contract-version-review, and
//! no-unsafe-fallback send-rule qualification records.
//!
//! This module owns the typed records that make a request's persisted-operation
//! binding a first-class, inspectable fact rather than hidden metadata. Each
//! detail row keeps the local operation name, the opaque remote id or hash, the
//! contract version it targets, the breaking-risk note, the binding/drift state,
//! and the open-contract action visible across the detail panel, request
//! composer, CLI/headless output, support export, and Help/About surfaces. When
//! a persisted-operation id or hash drifts, is deprecated, or is removed, the
//! companion drift-review sheets surface clear rerun, regenerate, and cancel
//! choices and block the send instead of silently falling back to raw local-text
//! execution. A raw send after a material persisted-operation mismatch is only
//! ever reachable through an explicit, acknowledged reviewed-downgrade choice.
//!
//! These records reuse the canonical matrix vocabulary ([`ContractKind`],
//! [`PersistedOperationBindingState`], [`RetentionMode`]) and reference the
//! [`freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix`](crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix)
//! packet as a verified upstream truth rather than minting a local synonym set.
//! The finer [`PersistedOperationDriftClass`] adds the hash-versus-id-versus
//! deprecation distinction this lane requires while mapping one-to-one onto the
//! frozen [`PersistedOperationBindingState`] states.
//!
//! Raw operation text, raw secrets, raw request bodies, raw headers, and raw
//! schema payloads do not belong in these records. Detail rows carry opaque,
//! non-secret id/hash and version labels, closed posture vocabularies, and
//! reviewable summaries. Server-bound identity is never hidden when a request
//! depends on it; persisted-operation drift never silently falls back to raw
//! execution; and the rerun/compare UX never widens request-history retention
//! toward unsafe body or header capture.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix::{
    ContractKind, PersistedOperationBindingState, RetentionMode,
    API_MATRIX_QUALIFICATION_RECORD_KIND,
};

/// Supported schema version for persisted-operation detail qualification packets.
pub const PERSISTED_OP_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`PersistedOpQualificationPacket`].
pub const PERSISTED_OP_QUALIFICATION_RECORD_KIND: &str =
    "add_persisted_operation_detail_hash_or_id_drift_checks_contract_version_review_and_no_unsafe_fallback_send_rules";

/// Repo-relative path to the checked-in persisted-operation detail packet.
pub const PERSISTED_OP_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.json";

/// Embedded checked-in packet JSON.
pub const PERSISTED_OP_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.json"
));

/// Qualification label shown on promoted persisted-operation surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistedOpQualificationLabel {
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

impl PersistedOpQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Persisted-operation consumer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistedOpSurfaceKind {
    /// Persisted-operation detail panel for a bound request.
    PersistedOperationDetail,
    /// Drift-review sheet shown before a drifted or deprecated send.
    DriftReviewSheet,
    /// Request composer where the send rule is enforced before dispatch.
    RequestComposer,
    /// CLI or headless request execution output.
    CliHeadlessOutput,
    /// Support-export bundle carrying persisted-operation truth.
    SupportExport,
    /// Help/About surface describing the persisted-operation and drift contract.
    HelpAbout,
}

/// Fine-grained binding/drift class of a persisted-operation detail.
///
/// This vocabulary distinguishes hash drift, id drift, deprecation, and removal
/// that the coarser frozen [`PersistedOperationBindingState`] collapses; it maps
/// one-to-one onto the frozen states via
/// [`PersistedOperationDriftClass::canonical_binding_state`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistedOperationDriftClass {
    /// Operation text and persisted id/hash both match; binding is current.
    Current,
    /// Server marks the still-resolvable persisted operation as deprecated.
    Deprecated,
    /// Operation text changed, so its hash no longer matches the persisted id.
    HashDrift,
    /// The server-side persisted id rotated and no longer matches the binding.
    IdDrift,
    /// The server no longer recognizes the persisted id.
    Removed,
}

impl PersistedOperationDriftClass {
    /// Returns true when the class must trigger a review before the request is
    /// sent (every class except [`Self::Current`]).
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::Current)
    }

    /// Returns true when the persisted-operation binding changed materially, so
    /// the bound id no longer resolves and a raw send would be an unsafe
    /// fallback. Deprecation still resolves and is therefore not a mismatch.
    pub const fn is_material_mismatch(self) -> bool {
        matches!(self, Self::HashDrift | Self::IdDrift | Self::Removed)
    }

    /// Returns the canonical frozen [`PersistedOperationBindingState`] this class
    /// resolves under, keeping the finer class aligned with the matrix.
    pub const fn canonical_binding_state(self) -> PersistedOperationBindingState {
        match self {
            Self::Current | Self::Deprecated => PersistedOperationBindingState::BoundCurrent,
            Self::HashDrift | Self::IdDrift | Self::Removed => {
                PersistedOperationBindingState::PersistedOperationDrift
            }
        }
    }
}

/// The enforced send decision for a persisted-operation detail before review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendDecisionClass {
    /// Send the bound persisted operation; only a current binding qualifies.
    SendPersistedBound,
    /// Block the send until the drift or deprecation is reviewed.
    BlockPendingReview,
}

impl SendDecisionClass {
    /// Returns true only when the request may be sent without a review step.
    pub const fn permits_send_without_review(self) -> bool {
        matches!(self, Self::SendPersistedBound)
    }
}

/// A single review choice a drift-review sheet can offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewChoiceKind {
    /// Re-send the reviewed binding (after acknowledging deprecation or drift).
    RerunReviewedBinding,
    /// Regenerate the persisted id/hash from the current operation text.
    RegeneratePersistedId,
    /// Cancel the send.
    Cancel,
    /// Explicit, acknowledged downgrade to raw local-text execution.
    ReviewedRawDowngrade,
}

impl ReviewChoiceKind {
    /// Returns true for the only choice that may produce raw execution after a
    /// persisted-operation mismatch.
    pub const fn is_explicit_downgrade(self) -> bool {
        matches!(self, Self::ReviewedRawDowngrade)
    }
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpQualificationProof {
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

/// Boolean guard set that keeps stable persisted-operation surfaces honest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpSurfaceGuardSet {
    /// The local operation name is visible.
    pub local_name_visible: bool,
    /// The opaque server-bound id or hash is visible.
    pub server_bound_id_visible: bool,
    /// The contract version is visible.
    pub contract_version_visible: bool,
    /// The breaking-risk note is visible when the binding requires review.
    pub breaking_risk_visible: bool,
    /// The open-contract action is visible.
    pub open_contract_action_visible: bool,
    /// The binding/drift state is visible.
    pub drift_state_visible: bool,
    /// The rerun/regenerate/cancel review choices are visible on drift.
    pub review_choices_visible: bool,
    /// Drift never silently falls back to raw request execution.
    pub no_silent_raw_fallback: bool,
    /// Compare and rerun UX never widens history toward unsafe body/header retention.
    pub no_unsafe_retention_for_compare: bool,
    /// Server-bound identity is never hidden when a request depends on it.
    pub server_bound_identity_never_hidden: bool,
}

impl PersistedOpSurfaceGuardSet {
    /// Returns true when every required guard is present.
    pub const fn all_visible(&self) -> bool {
        self.local_name_visible
            && self.server_bound_id_visible
            && self.contract_version_visible
            && self.breaking_risk_visible
            && self.open_contract_action_visible
            && self.drift_state_visible
            && self.review_choices_visible
            && self.no_silent_raw_fallback
            && self.no_unsafe_retention_for_compare
            && self.server_bound_identity_never_hidden
    }
}

/// One governed persisted-operation-consumer surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: PersistedOpSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: PersistedOpQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: PersistedOpQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<PersistedOpQualificationProof>,
    /// Visible guard set.
    pub guards: PersistedOpSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One persisted-operation detail row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpDetailRow {
    /// Stable detail id.
    pub detail_id: String,
    /// Owning surface ref.
    pub surface_ref: String,
    /// Contract family the operation targets.
    pub contract_kind: ContractKind,
    /// Matrix persisted-operation binding row this detail reflects.
    pub matrix_binding_ref: String,
    /// Opaque contract ref the operation belongs to.
    pub contract_ref: String,
    /// Local operation name.
    pub local_name: String,
    /// Opaque remote persisted id or hash ref (not raw operation text).
    pub remote_id_or_hash_ref: String,
    /// Contract version this binding targets.
    pub contract_version_label: String,
    /// Fine-grained binding/drift class.
    pub drift_class: PersistedOperationDriftClass,
    /// Canonical frozen binding state (must equal the class's canonical state).
    pub binding_state: PersistedOperationBindingState,
    /// Plain-language breaking-risk note; required when the class needs review.
    pub breaking_risk_note: String,
    /// Enforced send decision before any review.
    pub send_decision: SendDecisionClass,
    /// Whether the local operation name is visible.
    pub local_name_visible: bool,
    /// Whether the server-bound id or hash is visible.
    pub server_bound_id_visible: bool,
    /// Whether the contract version is visible.
    pub contract_version_visible: bool,
    /// Whether the breaking-risk note is visible.
    pub breaking_risk_visible: bool,
    /// Whether the open-contract action is available.
    pub open_contract_action_available: bool,
    /// Whether the binding/drift state is visible.
    pub drift_state_visible: bool,
    /// Whether drift is blocked from silently falling back to raw execution.
    pub no_silent_raw_fallback: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One drift-review sheet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpDriftReviewSheetRow {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Persisted-operation detail row this sheet reviews.
    pub detail_ref: String,
    /// Drift class the sheet reflects (must equal the detail's class).
    pub drift_class: PersistedOperationDriftClass,
    /// Opaque label of the previously bound id/hash.
    pub prior_id_or_hash_label: String,
    /// Opaque label of the now-resolved id/hash.
    pub resolved_id_or_hash_label: String,
    /// Contract version the binding targeted before the change.
    pub contract_version_from_label: String,
    /// Contract version the binding resolves to now.
    pub contract_version_to_label: String,
    /// Whether the sheet offers a rerun-reviewed-binding choice.
    pub offers_rerun_choice: bool,
    /// Whether the sheet offers a regenerate-persisted-id choice.
    pub offers_regenerate_choice: bool,
    /// Whether the sheet offers a cancel choice.
    pub offers_cancel_choice: bool,
    /// Whether review is required before the request is sent.
    pub requires_review_before_send: bool,
    /// Whether the send is blocked until the change is reviewed.
    pub send_blocked_until_reviewed: bool,
    /// Whether drift is blocked from silently falling back to raw execution.
    pub no_silent_raw_fallback: bool,
    /// History retention mode the review relies on.
    pub history_retention_mode: RetentionMode,
    /// Whether the sheet forces unsafe body/header retention to support compare.
    pub forces_unsafe_body_header_retention: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One enumerated review-choice row attached to a drift-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpReviewChoiceRow {
    /// Stable choice id.
    pub choice_id: String,
    /// Drift-review sheet this choice belongs to.
    pub sheet_ref: String,
    /// What the choice does.
    pub choice_kind: ReviewChoiceKind,
    /// Whether the choice requires an explicit acknowledgement.
    pub requires_ack: bool,
    /// Whether selecting the choice results in raw local-text execution.
    pub results_in_raw_execution: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Reference to an upstream M5 packet this row consumes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpUpstreamRefRow {
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

/// Summary counts for a persisted-operation detail qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of persisted-operation detail rows.
    pub detail_count: usize,
    /// Number of detail rows whose class requires review.
    pub drifted_detail_count: usize,
    /// Number of detail rows whose binding changed materially.
    pub material_mismatch_count: usize,
    /// Number of drift-review sheet rows.
    pub review_sheet_count: usize,
    /// Number of sheets that block the send until reviewed.
    pub send_blocked_sheet_count: usize,
    /// Number of review-choice rows.
    pub review_choice_count: usize,
    /// Number of explicit reviewed-downgrade choices.
    pub explicit_downgrade_count: usize,
    /// Number of upstream reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical persisted-operation detail qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOpQualificationPacket {
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
    pub surfaces: Vec<PersistedOpSurfaceQualificationRow>,
    /// Persisted-operation detail rows.
    pub details: Vec<PersistedOpDetailRow>,
    /// Drift-review sheet rows.
    pub review_sheets: Vec<PersistedOpDriftReviewSheetRow>,
    /// Review-choice rows.
    pub review_choices: Vec<PersistedOpReviewChoiceRow>,
    /// Upstream reference rows.
    pub upstream_refs: Vec<PersistedOpUpstreamRefRow>,
    /// Summary counts.
    pub summary: PersistedOpQualificationSummary,
}

impl PersistedOpQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> PersistedOpQualificationSummary {
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
        let drifted_detail_count = self
            .details
            .iter()
            .filter(|row| row.drift_class.requires_review())
            .count();
        let material_mismatch_count = self
            .details
            .iter()
            .filter(|row| row.drift_class.is_material_mismatch())
            .count();
        let send_blocked_sheet_count = self
            .review_sheets
            .iter()
            .filter(|row| row.send_blocked_until_reviewed)
            .count();
        let explicit_downgrade_count = self
            .review_choices
            .iter()
            .filter(|row| row.choice_kind.is_explicit_downgrade())
            .count();
        let integration_pass_count = self
            .upstream_refs
            .iter()
            .filter(|ref_row| ref_row.integration_verified)
            .count();
        PersistedOpQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            detail_count: self.details.len(),
            drifted_detail_count,
            material_mismatch_count,
            review_sheet_count: self.review_sheets.len(),
            send_blocked_sheet_count,
            review_choice_count: self.review_choices.len(),
            explicit_downgrade_count,
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of detail rows whose binding requires review (drift or
    /// deprecation), so the send is never silent.
    pub fn drifted_detail_ids(&self) -> Vec<String> {
        self.details
            .iter()
            .filter(|row| row.drift_class.requires_review())
            .map(|row| row.detail_id.clone())
            .collect()
    }

    /// Returns the ids of detail rows whose binding changed materially, where a
    /// raw send would be an unsafe fallback.
    pub fn material_mismatch_detail_ids(&self) -> Vec<String> {
        self.details
            .iter()
            .filter(|row| row.drift_class.is_material_mismatch())
            .map(|row| row.detail_id.clone())
            .collect()
    }

    /// Returns the ids of sheets that block the send until the change is reviewed.
    pub fn send_blocked_sheet_ids(&self) -> Vec<String> {
        self.review_sheets
            .iter()
            .filter(|row| row.send_blocked_until_reviewed)
            .map(|row| row.sheet_id.clone())
            .collect()
    }

    /// Returns the ids of explicit reviewed-downgrade choices, the only path to
    /// a raw send after a persisted-operation mismatch.
    pub fn explicit_downgrade_choice_ids(&self) -> Vec<String> {
        self.review_choices
            .iter()
            .filter(|row| row.choice_kind.is_explicit_downgrade())
            .map(|row| row.choice_id.clone())
            .collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<PersistedOpQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != PERSISTED_OP_QUALIFICATION_SCHEMA_VERSION {
            violations.push(PersistedOpQualificationViolation::SchemaVersion {
                expected: PERSISTED_OP_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PERSISTED_OP_QUALIFICATION_RECORD_KIND {
            violations.push(PersistedOpQualificationViolation::RecordKind {
                expected: PERSISTED_OP_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let surface_ids = collect_ids(
            self.surfaces.iter().map(|row| row.surface_id.as_str()),
            &mut violations,
            PersistedOpQualificationViolationKind::Surface,
        );
        let detail_ids = collect_ids(
            self.details.iter().map(|row| row.detail_id.as_str()),
            &mut violations,
            PersistedOpQualificationViolationKind::Detail,
        );
        let sheet_ids = collect_ids(
            self.review_sheets.iter().map(|row| row.sheet_id.as_str()),
            &mut violations,
            PersistedOpQualificationViolationKind::ReviewSheet,
        );
        collect_ids(
            self.review_choices.iter().map(|row| row.choice_id.as_str()),
            &mut violations,
            PersistedOpQualificationViolationKind::ReviewChoice,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            PersistedOpQualificationViolationKind::UpstreamRef,
        );

        self.validate_surfaces(&mut violations);
        self.validate_details(&mut violations, &surface_ids);
        self.validate_review_sheets(&mut violations, &detail_ids);
        self.validate_review_choices(&mut violations, &sheet_ids);
        self.validate_upstream_refs(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(PersistedOpQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_surfaces(&self, violations: &mut Vec<PersistedOpQualificationViolation>) {
        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        PersistedOpQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        PersistedOpQualificationViolation::StableSurfaceMissingGuard {
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
                    PersistedOpQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let surface_kinds: BTreeSet<_> = self.surfaces.iter().map(|row| row.surface_kind).collect();
        for required_kind in [
            PersistedOpSurfaceKind::PersistedOperationDetail,
            PersistedOpSurfaceKind::DriftReviewSheet,
            PersistedOpSurfaceKind::RequestComposer,
            PersistedOpSurfaceKind::CliHeadlessOutput,
            PersistedOpSurfaceKind::SupportExport,
            PersistedOpSurfaceKind::HelpAbout,
        ] {
            if !surface_kinds.contains(&required_kind) {
                violations.push(PersistedOpQualificationViolation::MissingSurfaceKind {
                    surface_kind: required_kind,
                });
            }
        }
    }

    fn validate_details(
        &self,
        violations: &mut Vec<PersistedOpQualificationViolation>,
        surface_ids: &BTreeSet<String>,
    ) {
        for row in &self.details {
            if !surface_ids.contains(&row.surface_ref)
                || row.matrix_binding_ref.is_empty()
                || row.contract_ref.is_empty()
                || row.local_name.is_empty()
                || row.remote_id_or_hash_ref.is_empty()
                || row.contract_version_label.is_empty()
                || !row.local_name_visible
                || !row.server_bound_id_visible
                || !row.contract_version_visible
                || !row.open_contract_action_available
                || !row.drift_state_visible
            {
                violations.push(PersistedOpQualificationViolation::IncompleteDetail {
                    detail_id: row.detail_id.clone(),
                });
            }
            // The fine drift class must resolve under its canonical frozen state
            // so the finer vocabulary never diverges from the matrix.
            if row.binding_state != row.drift_class.canonical_binding_state() {
                violations.push(PersistedOpQualificationViolation::DetailStateMismatch {
                    detail_id: row.detail_id.clone(),
                });
            }
            // Server-bound identity is never hidden when a request depends on it.
            if !row.server_bound_id_visible {
                violations.push(
                    PersistedOpQualificationViolation::ServerBoundIdentityHidden {
                        detail_id: row.detail_id.clone(),
                    },
                );
            }
            // Drift must never silently fall back to raw request execution.
            if !row.no_silent_raw_fallback {
                violations.push(
                    PersistedOpQualificationViolation::DetailAllowsSilentRawFallback {
                        detail_id: row.detail_id.clone(),
                    },
                );
            }
            // The enforced send decision follows the drift class: only a current
            // binding may send without review; everything else blocks first.
            let send_decision_ok = if row.drift_class.requires_review() {
                row.send_decision == SendDecisionClass::BlockPendingReview
            } else {
                row.send_decision == SendDecisionClass::SendPersistedBound
            };
            if !send_decision_ok {
                violations.push(
                    PersistedOpQualificationViolation::DetailSendDecisionInvalid {
                        detail_id: row.detail_id.clone(),
                    },
                );
            }
            // A class that requires review must show a breaking-risk note and be
            // backed by a drift-review sheet that blocks the send when the change
            // is material.
            if row.drift_class.requires_review() {
                if !row.breaking_risk_visible || row.breaking_risk_note.is_empty() {
                    violations.push(
                        PersistedOpQualificationViolation::DriftedDetailMissingRisk {
                            detail_id: row.detail_id.clone(),
                        },
                    );
                }
                let reviewed = self.review_sheets.iter().any(|sheet| {
                    sheet.detail_ref == row.detail_id
                        && sheet.requires_review_before_send
                        && (!row.drift_class.is_material_mismatch()
                            || sheet.send_blocked_until_reviewed)
                });
                if !reviewed {
                    violations.push(
                        PersistedOpQualificationViolation::DriftedDetailLacksReview {
                            detail_id: row.detail_id.clone(),
                        },
                    );
                }
            }
        }

        let drift_classes: BTreeSet<_> = self.details.iter().map(|row| row.drift_class).collect();
        for required_class in [
            PersistedOperationDriftClass::Current,
            PersistedOperationDriftClass::Deprecated,
            PersistedOperationDriftClass::HashDrift,
            PersistedOperationDriftClass::IdDrift,
            PersistedOperationDriftClass::Removed,
        ] {
            if !drift_classes.contains(&required_class) {
                violations.push(PersistedOpQualificationViolation::MissingDriftClass {
                    drift_class: required_class,
                });
            }
        }

        let send_decisions: BTreeSet<_> =
            self.details.iter().map(|row| row.send_decision).collect();
        for required_decision in [
            SendDecisionClass::SendPersistedBound,
            SendDecisionClass::BlockPendingReview,
        ] {
            if !send_decisions.contains(&required_decision) {
                violations.push(PersistedOpQualificationViolation::MissingSendDecision {
                    send_decision: required_decision,
                });
            }
        }

        // At least one binding must have changed materially so the no-unsafe
        // fallback review is exercised rather than asserted as possible.
        if !self
            .details
            .iter()
            .any(|row| row.drift_class.is_material_mismatch())
        {
            violations.push(PersistedOpQualificationViolation::NoMaterialMismatchCovered);
        }
    }

    fn validate_review_sheets(
        &self,
        violations: &mut Vec<PersistedOpQualificationViolation>,
        detail_ids: &BTreeSet<String>,
    ) {
        for row in &self.review_sheets {
            if !detail_ids.contains(&row.detail_ref)
                || row.prior_id_or_hash_label.is_empty()
                || row.resolved_id_or_hash_label.is_empty()
                || row.contract_version_from_label.is_empty()
                || row.contract_version_to_label.is_empty()
                || !row.offers_rerun_choice
                || !row.offers_regenerate_choice
                || !row.offers_cancel_choice
                || !row.requires_review_before_send
            {
                violations.push(PersistedOpQualificationViolation::IncompleteReviewSheet {
                    sheet_id: row.sheet_id.clone(),
                });
            }
            // A review sheet only reviews a class that requires review.
            if !row.drift_class.requires_review() {
                violations.push(
                    PersistedOpQualificationViolation::ReviewSheetOnCurrentBinding {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            // The sheet's class must match the detail it reviews.
            let detail_matches = self.details.iter().any(|detail| {
                detail.detail_id == row.detail_ref && detail.drift_class == row.drift_class
            });
            if !detail_matches {
                violations.push(
                    PersistedOpQualificationViolation::ReviewSheetClassMismatch {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            // A material mismatch must block the send until reviewed.
            if row.drift_class.is_material_mismatch() && !row.send_blocked_until_reviewed {
                violations.push(
                    PersistedOpQualificationViolation::MaterialMismatchNotBlocked {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            // Drift never silently falls back to raw execution.
            if !row.no_silent_raw_fallback {
                violations.push(
                    PersistedOpQualificationViolation::ReviewSheetAllowsSilentRawFallback {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            // Review and compare UX never widens history toward unsafe capture.
            if row.forces_unsafe_body_header_retention
                || row.history_retention_mode == RetentionMode::OptInFullCapture
            {
                violations.push(
                    PersistedOpQualificationViolation::ReviewSheetForcesUnsafeRetention {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            // A material mismatch may only reach raw execution through an explicit
            // reviewed-downgrade choice attached to this sheet.
            if row.drift_class.is_material_mismatch() {
                let has_downgrade = self.review_choices.iter().any(|choice| {
                    choice.sheet_ref == row.sheet_id
                        && choice.choice_kind.is_explicit_downgrade()
                        && choice.requires_ack
                });
                if !has_downgrade {
                    violations.push(
                        PersistedOpQualificationViolation::MaterialMismatchLacksReviewedDowngrade {
                            sheet_id: row.sheet_id.clone(),
                        },
                    );
                }
            }
        }
    }

    fn validate_review_choices(
        &self,
        violations: &mut Vec<PersistedOpQualificationViolation>,
        sheet_ids: &BTreeSet<String>,
    ) {
        for row in &self.review_choices {
            if !sheet_ids.contains(&row.sheet_ref) || row.rationale.is_empty() {
                violations.push(PersistedOpQualificationViolation::IncompleteReviewChoice {
                    choice_id: row.choice_id.clone(),
                });
            }
            // Only the explicit reviewed-downgrade choice may result in raw
            // execution, and it must require an acknowledgement.
            if row.choice_kind.is_explicit_downgrade() {
                if !row.requires_ack || !row.results_in_raw_execution {
                    violations.push(
                        PersistedOpQualificationViolation::DowngradeChoiceNotExplicit {
                            choice_id: row.choice_id.clone(),
                        },
                    );
                }
            } else if row.results_in_raw_execution {
                violations.push(
                    PersistedOpQualificationViolation::NonDowngradeChoiceProducesRaw {
                        choice_id: row.choice_id.clone(),
                    },
                );
            }
        }

        let choice_kinds: BTreeSet<_> = self
            .review_choices
            .iter()
            .map(|row| row.choice_kind)
            .collect();
        for required_kind in [
            ReviewChoiceKind::RerunReviewedBinding,
            ReviewChoiceKind::RegeneratePersistedId,
            ReviewChoiceKind::Cancel,
            ReviewChoiceKind::ReviewedRawDowngrade,
        ] {
            if !choice_kinds.contains(&required_kind) {
                violations.push(PersistedOpQualificationViolation::MissingReviewChoiceKind {
                    choice_kind: required_kind,
                });
            }
        }
    }

    fn validate_upstream_refs(&self, violations: &mut Vec<PersistedOpQualificationViolation>) {
        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(PersistedOpQualificationViolation::IncompleteUpstreamRef {
                    ref_id: row.ref_id.clone(),
                });
            }
        }
        // The detail lane must consume the frozen API-collection matrix as a
        // verified upstream packet so the persisted-operation and drift lanes
        // stay aligned.
        let consumes_matrix = self.upstream_refs.iter().any(|row| {
            row.upstream_record_kind == API_MATRIX_QUALIFICATION_RECORD_KIND
                && row.integration_verified
        });
        if !consumes_matrix {
            violations.push(PersistedOpQualificationViolation::MatrixUpstreamNotIntegrated);
        }
    }
}

/// Loads the checked-in persisted-operation detail qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_persisted_operation_qualification(
) -> Result<PersistedOpQualificationPacket, serde_json::Error> {
    serde_json::from_str(PERSISTED_OP_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistedOpQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Persisted-operation detail rows.
    Detail,
    /// Drift-review sheet rows.
    ReviewSheet,
    /// Review-choice rows.
    ReviewChoice,
    /// Upstream reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<PersistedOpQualificationViolation>,
    kind: PersistedOpQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(PersistedOpQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for persisted-operation detail qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistedOpQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: PersistedOpQualificationViolationKind,
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
        surface_kind: PersistedOpSurfaceKind,
    },
    /// Detail row does not project persisted-operation truth everywhere.
    IncompleteDetail { detail_id: String },
    /// Detail's drift class does not resolve under its canonical frozen state.
    DetailStateMismatch { detail_id: String },
    /// Detail hides server-bound identity a request depends on.
    ServerBoundIdentityHidden { detail_id: String },
    /// Detail allows silent fallback to raw execution.
    DetailAllowsSilentRawFallback { detail_id: String },
    /// Detail's send decision does not follow its drift class.
    DetailSendDecisionInvalid { detail_id: String },
    /// Drifted or deprecated detail hides its breaking-risk note.
    DriftedDetailMissingRisk { detail_id: String },
    /// Drifted or deprecated detail has no review sheet blocking the send.
    DriftedDetailLacksReview { detail_id: String },
    /// Required drift class is missing.
    MissingDriftClass {
        drift_class: PersistedOperationDriftClass,
    },
    /// Required send decision is missing.
    MissingSendDecision { send_decision: SendDecisionClass },
    /// No binding changed materially, so the no-fallback review is not exercised.
    NoMaterialMismatchCovered,
    /// Review sheet is incomplete or does not offer the required choices.
    IncompleteReviewSheet { sheet_id: String },
    /// Review sheet reviews a current (non-review) binding.
    ReviewSheetOnCurrentBinding { sheet_id: String },
    /// Review sheet's class does not match the detail it reviews.
    ReviewSheetClassMismatch { sheet_id: String },
    /// Material mismatch sheet does not block the send until reviewed.
    MaterialMismatchNotBlocked { sheet_id: String },
    /// Review sheet allows silent fallback to raw execution.
    ReviewSheetAllowsSilentRawFallback { sheet_id: String },
    /// Review sheet forces unsafe retention to support compare.
    ReviewSheetForcesUnsafeRetention { sheet_id: String },
    /// Material mismatch sheet has no explicit reviewed-downgrade choice.
    MaterialMismatchLacksReviewedDowngrade { sheet_id: String },
    /// Review-choice row is incomplete.
    IncompleteReviewChoice { choice_id: String },
    /// Downgrade choice does not require acknowledgement or raw-execution truth.
    DowngradeChoiceNotExplicit { choice_id: String },
    /// Non-downgrade choice silently produces raw execution.
    NonDowngradeChoiceProducesRaw { choice_id: String },
    /// Required review-choice kind is missing.
    MissingReviewChoiceKind { choice_kind: ReviewChoiceKind },
    /// Upstream reference is incomplete.
    IncompleteUpstreamRef { ref_id: String },
    /// The detail lane does not consume the API-collection matrix as a verified upstream packet.
    MatrixUpstreamNotIntegrated,
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for PersistedOpQualificationViolation {
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
            Self::IncompleteDetail { detail_id } => {
                write!(
                    f,
                    "{detail_id} does not project persisted-operation truth everywhere"
                )
            }
            Self::DetailStateMismatch { detail_id } => {
                write!(
                    f,
                    "{detail_id} drift class does not match its canonical binding state"
                )
            }
            Self::ServerBoundIdentityHidden { detail_id } => {
                write!(
                    f,
                    "{detail_id} hides server-bound identity a request depends on"
                )
            }
            Self::DetailAllowsSilentRawFallback { detail_id } => {
                write!(f, "{detail_id} allows silent fallback to raw execution")
            }
            Self::DetailSendDecisionInvalid { detail_id } => {
                write!(
                    f,
                    "{detail_id} send decision does not follow its drift class"
                )
            }
            Self::DriftedDetailMissingRisk { detail_id } => {
                write!(
                    f,
                    "{detail_id} requires review without a visible breaking-risk note"
                )
            }
            Self::DriftedDetailLacksReview { detail_id } => {
                write!(
                    f,
                    "{detail_id} requires review without a drift-review sheet blocking the send"
                )
            }
            Self::MissingDriftClass { drift_class } => {
                write!(f, "drift class {drift_class:?} is not covered")
            }
            Self::MissingSendDecision { send_decision } => {
                write!(f, "send decision {send_decision:?} is not covered")
            }
            Self::NoMaterialMismatchCovered => {
                write!(
                    f,
                    "no binding changed materially, so the no-fallback review is not exercised"
                )
            }
            Self::IncompleteReviewSheet { sheet_id } => {
                write!(f, "{sheet_id} does not offer the required review choices")
            }
            Self::ReviewSheetOnCurrentBinding { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} reviews a current binding that needs no review"
                )
            }
            Self::ReviewSheetClassMismatch { sheet_id } => {
                write!(f, "{sheet_id} class does not match the detail it reviews")
            }
            Self::MaterialMismatchNotBlocked { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} is a material mismatch that does not block the send"
                )
            }
            Self::ReviewSheetAllowsSilentRawFallback { sheet_id } => {
                write!(f, "{sheet_id} allows silent fallback to raw execution")
            }
            Self::ReviewSheetForcesUnsafeRetention { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} forces unsafe body/header retention to compare"
                )
            }
            Self::MaterialMismatchLacksReviewedDowngrade { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} is a material mismatch without an explicit reviewed-downgrade choice"
                )
            }
            Self::IncompleteReviewChoice { choice_id } => {
                write!(
                    f,
                    "{choice_id} does not project review-choice truth everywhere"
                )
            }
            Self::DowngradeChoiceNotExplicit { choice_id } => {
                write!(
                    f,
                    "{choice_id} is a downgrade choice without an acknowledged raw-execution path"
                )
            }
            Self::NonDowngradeChoiceProducesRaw { choice_id } => {
                write!(f, "{choice_id} silently produces raw execution")
            }
            Self::MissingReviewChoiceKind { choice_kind } => {
                write!(f, "review-choice kind {choice_kind:?} is not covered")
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
                    "detail lane does not consume the API-collection matrix as a verified upstream packet"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for PersistedOpQualificationViolation {}
