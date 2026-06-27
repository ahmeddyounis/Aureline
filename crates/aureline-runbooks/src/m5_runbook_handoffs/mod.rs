//! Governed runbook **control-plane handoffs** — how a runbook pivot to a provider
//! console or browser surface stays an explicit, attributable Aureline transition.
//!
//! Runbooks routinely cross out of Aureline's governed plane into provider consoles
//! and browser surfaces. Left implicit, that pivot can read as a hidden escape from
//! Aureline truth, or a browser reference doc can quietly present itself as if it
//! were executable in-product control. This module freezes the handoff object model
//! that prevents both:
//!
//! - Every pivot is a first-class
//!   [handoff packet](crate::m5_runbook_governance::ControlPlaneHandoffPacket) naming
//!   its [destination class](crate::m5_runbook_governance::HandoffDestinationClass),
//!   the [reason](crate::m5_runbook_governance::HandoffReasonClass) for the pivot,
//!   the object identity it crosses to, a
//!   [return anchor](crate::m5_runbook_governance::ReturnAnchor) that keeps the
//!   initiating target and evidence identity intact, and any narrowed authority that
//!   applies on the far side.
//! - Every destination carries a
//!   [reference-plane state](crate::m5_runbook_governance::ReferencePlaneState):
//!   [`HandoffRequired`](crate::m5_runbook_governance::ReferencePlaneState::HandoffRequired)
//!   when the destination is the true (external) control plane, and
//!   [`ReferenceOnly`](crate::m5_runbook_governance::ReferencePlaneState::ReferenceOnly)
//!   when it is read-only browser documentation. A reference-only destination can
//!   never claim executable in-product control.
//!
//! The [`M5RunbookHandoffRegister`] is the one inspectable, serde-serializable truth
//! packet the consuming surfaces read. It projects every governed handoff — including
//! the live handoffs embedded in the operator-scenario execution records — into the
//! same surface-independent [projection](RunbookHandoffProjection), and publishes a
//! [reference-plane catalog](ReferencePlaneEntry) naming the browser-only reference
//! docs and provider consoles that remain the true control plane. The incident
//! workspace, operator history, support exports, and docs/help all render the same
//! truth, so a console/browser pivot is never a hidden escape and a reference doc
//! never masquerades as in-product control. The packet carries no credential bodies
//! or raw console/browser payloads.
//!
//! - Register schema:
//!   [`schemas/runbooks/m5-runbook-handoff-register.schema.json`](../../../../../schemas/runbooks/m5-runbook-handoff-register.schema.json)
//! - Contract doc:
//!   [`docs/runbooks/m5-runbook-handoffs.md`](../../../../../docs/runbooks/m5-runbook-handoffs.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_runbook_handoff_register, seeded_reference_plane_entries,
    seeded_runbook_handoff_packets, M5_RUNBOOK_HANDOFF_REGISTER_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_runbook_governance::{
    seeded_operator_scenario_records, ControlPlaneBoundaryClass, ControlPlaneHandoffPacket,
    HandoffDestinationClass, HandoffReasonClass, ReferencePlaneState, ReturnAnchorObjectClass,
};

/// Record-kind tag carried by [`M5RunbookHandoffRegister`].
pub const M5_RUNBOOK_HANDOFF_REGISTER_RECORD_KIND: &str = "m5_runbook_handoff_register";

/// Record-kind tag carried by [`ReferencePlaneEntry`].
pub const M5_RUNBOOK_REFERENCE_PLANE_RECORD_KIND: &str = "m5_runbook_reference_plane_entry";

/// Schema version shared by the register and its embedded objects.
pub const M5_RUNBOOK_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the handoff-register schema.
pub const M5_RUNBOOK_HANDOFF_REGISTER_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-handoff-register.schema.json";

/// Repo-relative path of the published handoff-register inventory.
pub const M5_RUNBOOK_HANDOFF_REGISTER_REF: &str =
    "artifacts/runbooks/m5-runbook-handoff-register.json";

/// Repo-relative path of the release-grade handoff-register export.
pub const M5_RUNBOOK_HANDOFF_REGISTER_PROOF_REF: &str =
    "artifacts/release/m5-runbook-proof/runbook-handoff-register.json";

/// Repo-relative path of the handoff-register contract doc.
pub const M5_RUNBOOK_HANDOFF_DOC_REF: &str = "docs/runbooks/m5-runbook-handoffs.md";

/// Repo-relative directory of the per-handoff projection fixtures.
pub const M5_RUNBOOK_HANDOFF_FIXTURE_DIR: &str = "fixtures/runbooks/m5-handoff-packets/";

/// Prefix every register-owned message id carries so consumers can route it. The
/// embedded handoff packets keep the governance lane's prefix, since they are
/// governance objects; the register's own objects use this one.
pub const M5_RUNBOOK_HANDOFF_MESSAGE_ID_PREFIX: &str = "runbooks_handoffs.";

/// A surface that renders the runbook handoff register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookHandoffSurface {
    /// The incident workspace.
    IncidentWorkspace,
    /// The operator execution-history view.
    OperatorHistory,
    /// Support exports / bundles.
    SupportExport,
    /// Docs and Help surfaces.
    DocsHelp,
}

impl RunbookHandoffSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::IncidentWorkspace,
        Self::OperatorHistory,
        Self::SupportExport,
        Self::DocsHelp,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentWorkspace => "incident_workspace",
            Self::OperatorHistory => "operator_history",
            Self::SupportExport => "support_export",
            Self::DocsHelp => "docs_help",
        }
    }
}

/// The surface-independent rendered truth for one governed handoff. Every consuming
/// surface shows the same explicit, attributable transition: where it goes, why,
/// whether the destination is read-only reference or the true control plane, and the
/// return path that keeps the initiating target and evidence identity intact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookHandoffProjection {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Destination-class token.
    pub destination_class: String,
    /// Reason-class token.
    pub reason_class: String,
    /// Control-plane boundary token.
    pub boundary_class: String,
    /// Reference-plane state token.
    pub reference_plane_state: String,
    /// Opaque, redaction-safe ref to the handoff target (console id / browser route).
    pub target_ref: String,
    /// Opaque, redaction-safe ref to the destination object identity crossed to.
    pub destination_object_ref: String,
    /// Opaque ref attributing the handoff to a session/actor.
    pub attribution_ref: String,
    /// Return-anchor object-class token.
    pub return_initiating_object_class: String,
    /// Opaque ref to the Aureline object the pivot returns to.
    pub return_initiating_object_ref: String,
    /// The target identity preserved across the pivot.
    pub return_target_continuity_ref: String,
    /// The evidence identity preserved across the pivot.
    pub return_evidence_continuity_ref: String,
    /// Stable message id for the "return to Aureline" affordance.
    pub return_message_id: String,
    /// Message id naming any authority narrowed on the far side, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_authority_message_id: Option<String>,
    /// Whether control returns to Aureline's governed plane after the pivot.
    pub returns_to_governed_plane: bool,
    /// Whether the destination is the true (external) control plane.
    pub is_true_control_plane: bool,
    /// Whether the destination is read-only reference documentation.
    pub is_reference_only: bool,
    /// Whether the destination is executable *in product*. Always false: the control
    /// plane is external or the destination is reference-only, never in-app.
    pub executable_in_product: bool,
    /// Whether the pivot is attributable (named target/attribution + return anchor).
    pub attributable: bool,
    /// Whether the handoff would mint a hidden privileged mutate channel; must be false.
    pub creates_hidden_mutate_channel: bool,
    /// Stable message id; mirrors the embedded packet's id.
    pub detail_message_id: String,
}

impl RunbookHandoffProjection {
    /// Projects one governed handoff packet into its surface-independent projection.
    pub fn derive(packet: &ControlPlaneHandoffPacket) -> Self {
        let anchor = &packet.return_anchor;
        Self {
            handoff_id: packet.handoff_id.clone(),
            destination_class: packet.destination_class.as_str().to_owned(),
            reason_class: packet.reason_class.as_str().to_owned(),
            boundary_class: packet.boundary_class.as_str().to_owned(),
            reference_plane_state: packet.reference_plane_state.as_str().to_owned(),
            target_ref: packet.target_ref.clone(),
            destination_object_ref: packet.destination_object_ref.clone(),
            attribution_ref: packet.attribution_ref.clone(),
            return_initiating_object_class: anchor.initiating_object_class.as_str().to_owned(),
            return_initiating_object_ref: anchor.initiating_object_ref.clone(),
            return_target_continuity_ref: anchor.target_continuity_ref.clone(),
            return_evidence_continuity_ref: anchor.evidence_continuity_ref.clone(),
            return_message_id: anchor.return_message_id.clone(),
            narrowed_authority_message_id: packet.narrowed_authority_message_id.clone(),
            returns_to_governed_plane: packet.returns_to_governed_plane,
            is_true_control_plane: packet.reference_plane_state.is_handoff_required(),
            is_reference_only: packet.reference_plane_state.is_reference_only(),
            // A handoff is, by definition, a pivot *out of* the governed plane, so it
            // is never in-product executable control.
            executable_in_product: false,
            attributable: !packet.attribution_ref.trim().is_empty() && anchor.is_complete(),
            creates_hidden_mutate_channel: packet.creates_hidden_mutate_channel,
            detail_message_id: packet.detail_message_id.clone(),
        }
    }
}

/// One catalog entry for a destination that remains the true control plane. The
/// catalog makes explicit which browser-only reference docs and provider consoles
/// are reference-only and which are handoff-required, so a reference doc never reads
/// as in-product control and a console pivot is always an explicit transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencePlaneEntry {
    /// Record kind; must equal [`M5_RUNBOOK_REFERENCE_PLANE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_HANDOFF_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable entry id.
    pub entry_id: String,
    /// Reviewer-facing label.
    pub entry_label: String,
    /// What kind of destination this entry describes.
    pub destination_class: HandoffDestinationClass,
    /// Whether the destination is the true control plane, read-only reference, or in-app.
    pub reference_plane_state: ReferencePlaneState,
    /// Whether the destination is the true (external) control plane (handoff-required).
    pub is_true_control_plane: bool,
    /// Whether the destination is read-only reference documentation.
    pub is_reference_only: bool,
    /// Whether the destination is executable *in product*. Always false: a true
    /// control plane is external and a reference doc is read-only.
    pub executable_in_product: bool,
    /// Stable message id naming the authority posture; prefixed
    /// [`M5_RUNBOOK_HANDOFF_MESSAGE_ID_PREFIX`].
    pub authority_note_message_id: String,
    /// Stable message id; prefixed [`M5_RUNBOOK_HANDOFF_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl ReferencePlaneEntry {
    /// Validates one reference-plane entry's invariants.
    pub fn validate(&self) -> Vec<M5RunbookHandoffViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_REFERENCE_PLANE_RECORD_KIND
            || self.schema_version != M5_RUNBOOK_HANDOFF_SCHEMA_VERSION
        {
            out.push(M5RunbookHandoffViolation::WrongEntryRecordKind);
        }
        if self.entry_id.trim().is_empty() || self.entry_label.trim().is_empty() {
            out.push(M5RunbookHandoffViolation::MissingIdentity);
        }
        if !self
            .authority_note_message_id
            .starts_with(M5_RUNBOOK_HANDOFF_MESSAGE_ID_PREFIX)
            || !self
                .detail_message_id
                .starts_with(M5_RUNBOOK_HANDOFF_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookHandoffViolation::UnprefixedMessageId);
        }
        // The destination class fixes the reference-plane state; the entry cannot lie.
        if self.destination_class.reference_plane_state() != self.reference_plane_state {
            out.push(M5RunbookHandoffViolation::ReferencePlaneStateMismatch);
        }
        if self.is_true_control_plane != self.reference_plane_state.is_handoff_required()
            || self.is_reference_only != self.reference_plane_state.is_reference_only()
        {
            out.push(M5RunbookHandoffViolation::ReferencePlaneStateMismatch);
        }
        // A reference-plane destination is never in-product executable control —
        // that is exactly the masquerade the catalog forbids.
        if self.executable_in_product {
            out.push(M5RunbookHandoffViolation::ReferenceOnlyClaimsInProductControl);
        }
        out
    }
}

/// Which surfaces expose the handoff register. Every flag must hold so a pivot reads
/// identically wherever it is rendered or exported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookHandoffSurfaceExposure {
    /// The incident workspace exposes the register.
    pub incident_workspace_exposes_handoffs: bool,
    /// The operator history exposes the register.
    pub operator_history_exposes_handoffs: bool,
    /// Support exports expose the register.
    pub support_export_exposes_handoffs: bool,
    /// Docs/help expose the register.
    pub docs_help_exposes_handoffs: bool,
}

impl RunbookHandoffSurfaceExposure {
    /// The canonical exposure: every surface renders the register.
    pub const fn all_surfaces() -> Self {
        Self {
            incident_workspace_exposes_handoffs: true,
            operator_history_exposes_handoffs: true,
            support_export_exposes_handoffs: true,
            docs_help_exposes_handoffs: true,
        }
    }

    /// True when every surface exposes the register.
    pub const fn all_expose(&self) -> bool {
        self.incident_workspace_exposes_handoffs
            && self.operator_history_exposes_handoffs
            && self.support_export_exposes_handoffs
            && self.docs_help_exposes_handoffs
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookHandoffVocabulary {
    /// Destination-class tokens.
    pub destination_classes: Vec<String>,
    /// Reason-class tokens.
    pub reason_classes: Vec<String>,
    /// Reference-plane state tokens.
    pub reference_plane_states: Vec<String>,
    /// Control-plane boundary tokens.
    pub control_plane_boundaries: Vec<String>,
    /// Return-anchor object-class tokens.
    pub return_anchor_object_classes: Vec<String>,
    /// Surface tokens.
    pub surfaces: Vec<String>,
}

impl RunbookHandoffVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            destination_classes: HandoffDestinationClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            reason_classes: HandoffReasonClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            reference_plane_states: ReferencePlaneState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            control_plane_boundaries: ControlPlaneBoundaryClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            return_anchor_object_classes: ReturnAnchorObjectClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            surfaces: RunbookHandoffSurface::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review for the handoff register. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookHandoffConformance {
    /// Every handoff names a destination, a reason, and a destination object identity.
    pub every_handoff_names_destination_reason_and_object_identity: bool,
    /// Every handoff carries a return anchor preserving the target and evidence identity.
    pub every_handoff_carries_return_anchor_preserving_target_and_evidence: bool,
    /// Reference-only destinations never present as executable in-product control.
    pub reference_only_destinations_never_present_as_in_product_control: bool,
    /// Console/browser pivots are explicit attributable transitions, not hidden escapes.
    pub console_browser_pivots_are_attributable_transitions: bool,
    /// No handoff mints a hidden privileged mutate channel.
    pub no_handoff_mints_a_hidden_mutate_channel: bool,
    /// Every handoff embedded in the operator-scenario records is represented here.
    pub operator_scenario_handoffs_are_represented: bool,
    /// The register is generated from the same checked-in handoff packets.
    pub generated_from_checked_in_handoffs: bool,
    /// The export carries no raw boundary material.
    pub export_carries_no_raw_boundary_material: bool,
}

impl RunbookHandoffConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_handoff_names_destination_reason_and_object_identity
            && self.every_handoff_carries_return_anchor_preserving_target_and_evidence
            && self.reference_only_destinations_never_present_as_in_product_control
            && self.console_browser_pivots_are_attributable_transitions
            && self.no_handoff_mints_a_hidden_mutate_channel
            && self.operator_scenario_handoffs_are_represented
            && self.generated_from_checked_in_handoffs
            && self.export_carries_no_raw_boundary_material
    }
}

/// Constructor input for [`M5RunbookHandoffRegister::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunbookHandoffRegisterInput {
    /// Stable register id.
    pub register_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the register was computed as-of.
    pub evaluated_at: String,
    /// The governed handoff packets.
    pub handoffs: Vec<ControlPlaneHandoffPacket>,
    /// The reference-plane catalog.
    pub reference_plane: Vec<ReferencePlaneEntry>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 runbook handoff register: the inventory of governed control-plane
/// handoffs, their surface-independent projections, and the reference-plane catalog
/// every consuming surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookHandoffRegister {
    /// Record kind; must equal [`M5_RUNBOOK_HANDOFF_REGISTER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_HANDOFF_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable register id.
    pub register_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the register was computed as-of.
    pub evaluated_at: String,
    /// The governed handoff packets.
    pub handoffs: Vec<ControlPlaneHandoffPacket>,
    /// One surface-independent projection per handoff, in handoff order.
    pub projections: Vec<RunbookHandoffProjection>,
    /// The reference-plane catalog of true-control-plane and reference-only destinations.
    pub reference_plane: Vec<ReferencePlaneEntry>,
    /// Which surfaces expose the register.
    pub surface_exposure: RunbookHandoffSurfaceExposure,
    /// Controlled-vocabulary set.
    pub vocabulary: RunbookHandoffVocabulary,
    /// Conformance review block.
    pub conformance: RunbookHandoffConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunbookHandoffRegister {
    /// Builds a register from seed input, deriving each handoff's projection and the
    /// conformance review from the handoff packets and reference-plane catalog.
    pub fn new(input: M5RunbookHandoffRegisterInput) -> Self {
        let projections: Vec<RunbookHandoffProjection> = input
            .handoffs
            .iter()
            .map(RunbookHandoffProjection::derive)
            .collect();
        let conformance = derive_conformance(&input.handoffs, &input.reference_plane);
        Self {
            record_kind: M5_RUNBOOK_HANDOFF_REGISTER_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_HANDOFF_SCHEMA_VERSION,
            register_id: input.register_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            handoffs: input.handoffs,
            projections,
            reference_plane: input.reference_plane,
            surface_exposure: RunbookHandoffSurfaceExposure::all_surfaces(),
            vocabulary: RunbookHandoffVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a handoff projection by id.
    pub fn projection(&self, handoff_id: &str) -> Option<&RunbookHandoffProjection> {
        self.projections.iter().find(|p| p.handoff_id == handoff_id)
    }

    /// The projections a given surface renders. Every surface shows the same truth.
    pub fn projections_for_surface(
        &self,
        _surface: RunbookHandoffSurface,
    ) -> Vec<RunbookHandoffProjection> {
        self.projections.clone()
    }

    /// Validates the register's invariants.
    pub fn validate(&self) -> Vec<M5RunbookHandoffViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_HANDOFF_REGISTER_RECORD_KIND {
            out.push(M5RunbookHandoffViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNBOOK_HANDOFF_SCHEMA_VERSION {
            out.push(M5RunbookHandoffViolation::WrongSchemaVersion);
        }
        if self.register_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5RunbookHandoffViolation::MissingIdentity);
        }
        if self.handoffs.is_empty() {
            out.push(M5RunbookHandoffViolation::RegisterHasNoHandoffs);
        }

        // Unique handoff ids, and every embedded packet passes governance validation.
        let mut seen = std::collections::BTreeSet::new();
        for packet in &self.handoffs {
            if !seen.insert(packet.handoff_id.as_str()) {
                out.push(M5RunbookHandoffViolation::DuplicateHandoffId);
            }
            if !packet.validate().is_empty() {
                out.push(M5RunbookHandoffViolation::HandoffPacketInvalid);
            }
        }

        // Unique reference-plane entry ids, and every entry validates.
        let mut seen_entries = std::collections::BTreeSet::new();
        for entry in &self.reference_plane {
            if !seen_entries.insert(entry.entry_id.as_str()) {
                out.push(M5RunbookHandoffViolation::DuplicateReferencePlaneId);
            }
            out.extend(entry.validate());
        }

        // The projections must recompute exactly from the handoff packets.
        let expected: Vec<RunbookHandoffProjection> = self
            .handoffs
            .iter()
            .map(RunbookHandoffProjection::derive)
            .collect();
        if expected != self.projections {
            out.push(M5RunbookHandoffViolation::ProjectionDrift);
        }

        if !self.surface_exposure.all_expose() {
            out.push(M5RunbookHandoffViolation::SurfaceExposureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5RunbookHandoffViolation::VocabularyMismatch);
        }
        if self.conformance != derive_conformance(&self.handoffs, &self.reference_plane)
            || !self.conformance.all_hold()
        {
            out.push(M5RunbookHandoffViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runbook handoff register serializes"),
        ) {
            out.push(M5RunbookHandoffViolation::RawBoundaryMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the register.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runbook handoff register serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Runbook Control-Plane Handoff Register\n\n");
        out.push_str(&format!("- Register: `{}`\n", self.register_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!("- Handoffs: {}\n", self.handoffs.len()));
        let control_plane = self
            .projections
            .iter()
            .filter(|p| p.is_true_control_plane)
            .count();
        let reference = self
            .projections
            .iter()
            .filter(|p| p.is_reference_only)
            .count();
        out.push_str(&format!(
            "- True control plane (handoff-required): {control_plane} · Reference-only: {reference}\n"
        ));
        out.push_str(
            "- Exposed on: incident workspace, operator history, support exports, docs/help\n",
        );

        out.push_str("\n## Governed handoffs\n\n");
        out.push_str(
            "| Handoff | Destination | Reason | Reference plane | Returns | Return anchor |\n",
        );
        out.push_str(
            "|---------|-------------|--------|-----------------|---------|---------------|\n",
        );
        for p in &self.projections {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | `{}` |\n",
                p.handoff_id,
                p.destination_class,
                p.reason_class,
                p.reference_plane_state,
                if p.returns_to_governed_plane {
                    "yes"
                } else {
                    "no"
                },
                p.return_initiating_object_ref,
            ));
        }

        out.push_str("\n## Reference-plane catalog\n\n");
        out.push_str("A reference-only destination can never present as in-product control.\n\n");
        out.push_str("| Destination | Class | Reference plane | In-product control |\n");
        out.push_str("|-------------|-------|-----------------|--------------------|\n");
        for entry in &self.reference_plane {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} |\n",
                entry.entry_id,
                entry.destination_class.as_str(),
                entry.reference_plane_state.as_str(),
                if entry.executable_in_product {
                    "yes"
                } else {
                    "no"
                },
            ));
        }
        out
    }
}

/// Derives the conformance review from the handoff packets and reference-plane
/// catalog so the stored block reflects the actual register rather than an assertion.
fn derive_conformance(
    handoffs: &[ControlPlaneHandoffPacket],
    reference_plane: &[ReferencePlaneEntry],
) -> RunbookHandoffConformance {
    let names_identity = !handoffs.is_empty()
        && handoffs.iter().all(|h| {
            !h.handoff_id.trim().is_empty()
                && !h.destination_object_ref.trim().is_empty()
                && !h.target_ref.trim().is_empty()
        });

    let carries_anchor = handoffs.iter().all(|h| h.return_anchor.is_complete());

    // A reference-only handoff never claims that control is exercised on the far
    // side, and every reference-plane entry is non-executable in-product.
    let reference_only_safe = handoffs.iter().all(|h| {
        !h.reference_plane_state.is_reference_only() || !h.reason_class.exercises_far_side_control()
    }) && reference_plane.iter().all(|e| !e.executable_in_product);

    let attributable_transitions = handoffs.iter().all(|h| {
        h.boundary_class.leaves_governed_plane()
            && !h.attribution_ref.trim().is_empty()
            && h.destination_class.boundary_class() == h.boundary_class
    });

    let no_hidden_mutate = handoffs.iter().all(|h| !h.creates_hidden_mutate_channel);

    // Every handoff embedded in the operator-scenario execution records is present
    // here, so the register cannot quietly drop a live pivot.
    let register_ids: std::collections::BTreeSet<&str> =
        handoffs.iter().map(|h| h.handoff_id.as_str()).collect();
    let scenario_represented = seeded_operator_scenario_records()
        .iter()
        .flat_map(|r| r.executed_steps.iter())
        .filter_map(|s| s.handoff.as_ref())
        .all(|h| register_ids.contains(h.handoff_id.as_str()));

    let generated = handoffs.iter().all(|h| h.validate().is_empty())
        && reference_plane.iter().all(|e| e.validate().is_empty());

    let export_clean = handoffs.iter().all(|h| !h.creates_hidden_mutate_channel);

    RunbookHandoffConformance {
        every_handoff_names_destination_reason_and_object_identity: names_identity,
        every_handoff_carries_return_anchor_preserving_target_and_evidence: carries_anchor,
        reference_only_destinations_never_present_as_in_product_control: reference_only_safe,
        console_browser_pivots_are_attributable_transitions: attributable_transitions,
        no_handoff_mints_a_hidden_mutate_channel: no_hidden_mutate,
        operator_scenario_handoffs_are_represented: scenario_represented,
        generated_from_checked_in_handoffs: generated,
        export_carries_no_raw_boundary_material: export_clean,
    }
}

/// Validation failures for the runbook handoff-register lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunbookHandoffViolation {
    /// The register record kind is wrong.
    WrongRecordKind,
    /// The register schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The register declares no handoffs.
    RegisterHasNoHandoffs,
    /// Two handoffs share a handoff id.
    DuplicateHandoffId,
    /// Two reference-plane entries share an entry id.
    DuplicateReferencePlaneId,
    /// An embedded handoff packet failed governance validation.
    HandoffPacketInvalid,
    /// A reference-plane entry carries the wrong record kind or schema version.
    WrongEntryRecordKind,
    /// A reference-plane entry's destination class and reference-plane state disagree.
    ReferencePlaneStateMismatch,
    /// A reference-only destination claims executable in-product control.
    ReferenceOnlyClaimsInProductControl,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The projections drifted from a fresh recompute.
    ProjectionDrift,
    /// A surface does not expose the register.
    SurfaceExposureIncomplete,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The conformance review does not hold or drifted.
    ConformanceReviewFailed,
    /// The export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RunbookHandoffViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::RegisterHasNoHandoffs => "register_has_no_handoffs",
            Self::DuplicateHandoffId => "duplicate_handoff_id",
            Self::DuplicateReferencePlaneId => "duplicate_reference_plane_id",
            Self::HandoffPacketInvalid => "handoff_packet_invalid",
            Self::WrongEntryRecordKind => "wrong_entry_record_kind",
            Self::ReferencePlaneStateMismatch => "reference_plane_state_mismatch",
            Self::ReferenceOnlyClaimsInProductControl => "reference_only_claims_in_product_control",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::ProjectionDrift => "projection_drift",
            Self::SurfaceExposureIncomplete => "surface_exposure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked boundary material.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden boundary material. Returns true when a
/// key (case-insensitive) contains a forbidden substring.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_boundary_material(child)
        }),
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
