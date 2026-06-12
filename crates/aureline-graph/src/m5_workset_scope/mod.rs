//! Canonical M5 workset-scope descriptor packet: the single scope snapshot every M5
//! code-understanding surface binds to so slice boundaries stay visible and durable.
//!
//! Where [`crate::m5_graph_governance`] freezes which depth claim each lane may publish,
//! this packet answers the prior question every graph-backed surface must answer first —
//! *what slice of the workspace am I actually looking at?* It carries one active
//! [`WorksetScopeSnapshot`] (a stable snapshot id, an as-of date, and the canonical
//! [`WorksetScopeDescriptor`] it bounds), the explicit [`ScopeChangeAction`]s that widen or
//! narrow that slice, and a [`ScopeConsumerBinding`] for every M5 code-understanding surface
//! — docs recall, topology views, the architecture explainer, review explanation, the
//! onboarding tour, and AI context assembly — so no surface leaves its scope boundary
//! implicit.
//!
//! The descriptor reuses the stable scope vocabulary from [`crate::explainers`]
//! ([`WorksetScopeMode`], [`WorksetScopeSource`], [`IndexCoverage`], and
//! [`WorksetScopeDescriptor`]) rather than minting separate workset hints for search,
//! explainers, review, onboarding, or AI context, so every surface narrows from one shared
//! model.
//!
//! Three invariants hold across the packet:
//!
//! - **No silent broadening.** A [`ScopeChangeAction`] that widens the slice
//!   ([`ScopeChangeDirection::Widen`]) is always reviewable
//!   ([`ScopeChangeAction::requires_review`] is `true`), and any merely
//!   [`ScopeChangeActuation::Suggested`] action is reviewable as well — a suggestion may
//!   exist, but a graph-backed feature may not silently broaden beyond the active slice.
//! - **No slice masquerading as the whole workspace.** A
//!   [`ScopeConsumerBinding::implies_full_workspace`] flag may only be `true` when the active
//!   descriptor is [`WorksetScopeMode::Full`], so an explainer, impact card, or AI-context
//!   binding never implies whole-workspace knowledge while it is bound to a sparse slice.
//! - **Replayable scope.** Every binding records the active [`WorksetScopeSnapshot::snapshot_id`]
//!   and the descriptor's [`WorksetScopeDescriptor::scope_id`], so a later support export or
//!   replay can reconstruct exactly which scope the user actually queried instead of guessing
//!   from result content.
//!
//! The packet binds upstream to the canonical graph-depth governance matrix and the
//! scope-provenance truth packet it extends, and exports release-evidence, help-surface,
//! docs-badge, and support-export refs so those surfaces narrow from one packet rather than
//! parallel spreadsheets.
//!
//! The packet is checked in at `artifacts/graph/m5/m5-workset-scope.json` and embedded here.
//! It is metadata-only: every field is a typed state, a count, a label, or an opaque ref, and
//! it carries no credential bodies, raw provider payloads, or graph node contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::explainers::{
    IndexCoverage, WorksetScopeDescriptor, WorksetScopeMode, WorksetScopeSource,
};

/// Supported M5 workset-scope packet schema version.
pub const M5_WORKSET_SCOPE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_WORKSET_SCOPE_RECORD_KIND: &str = "m5_workset_scope_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_WORKSET_SCOPE_PATH: &str = "artifacts/graph/m5/m5-workset-scope.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_WORKSET_SCOPE_SCHEMA_REF: &str = "schemas/graph/m5-workset-scope.schema.json";

/// Repo-relative path to the companion document.
pub const M5_WORKSET_SCOPE_DOC_REF: &str = "docs/graph/m5/m5-workset-scope.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_WORKSET_SCOPE_FIXTURE_DIR: &str = "fixtures/graph/m5/m5-workset-scope";

/// Repo-relative path to the canonical graph-depth governance matrix this packet extends.
pub const M5_WORKSET_SCOPE_GOVERNANCE_MATRIX_REF: &str =
    "artifacts/graph/m5/m5-graph-governance.json";

/// Repo-relative path to the canonical scope-provenance source packet this descriptor draws on.
pub const M5_WORKSET_SCOPE_SOURCE_PACKET_REF: &str =
    "artifacts/search/m4/scope_provenance_truth_packet.json";

/// Embedded checked-in packet JSON.
pub const M5_WORKSET_SCOPE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-workset-scope.json"
));

/// An M5 code-understanding surface that binds to the active workset-scope snapshot.
///
/// The closed vocabulary is exhaustive: every surface that can imply whole-workspace
/// knowledge carries a binding so its slice boundary stays explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetScopeConsumerSurface {
    /// Documentation recall over the workspace knowledge pack.
    DocsRecall,
    /// Topology node/edge views.
    TopologyView,
    /// The generated architecture explainer.
    ArchitectureExplainer,
    /// Review explanation of changed code.
    ReviewExplanation,
    /// The onboarding tour.
    OnboardingTour,
    /// AI context assembly.
    AiContextAssembly,
}

impl WorksetScopeConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DocsRecall,
        Self::TopologyView,
        Self::ArchitectureExplainer,
        Self::ReviewExplanation,
        Self::OnboardingTour,
        Self::AiContextAssembly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsRecall => "docs_recall",
            Self::TopologyView => "topology_view",
            Self::ArchitectureExplainer => "architecture_explainer",
            Self::ReviewExplanation => "review_explanation",
            Self::OnboardingTour => "onboarding_tour",
            Self::AiContextAssembly => "ai_context_assembly",
        }
    }
}

/// Whether a scope-change action widens or narrows the active slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeChangeDirection {
    /// Broaden the active slice toward the full workspace.
    Widen,
    /// Tighten the active slice to a smaller workset.
    Narrow,
}

impl ScopeChangeDirection {
    /// Every direction, in declaration order.
    pub const ALL: [Self; 2] = [Self::Widen, Self::Narrow];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Widen => "widen",
            Self::Narrow => "narrow",
        }
    }

    /// Whether this direction broadens the active slice and therefore must stay reviewable.
    pub const fn is_widen(self) -> bool {
        matches!(self, Self::Widen)
    }
}

/// How a scope-change action is actuated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeChangeActuation {
    /// An explicit user action that the product offers.
    Explicit,
    /// A suggestion the product surfaces; it is never auto-applied.
    Suggested,
}

impl ScopeChangeActuation {
    /// Every actuation, in declaration order.
    pub const ALL: [Self; 2] = [Self::Explicit, Self::Suggested];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explicit => "explicit",
            Self::Suggested => "suggested",
        }
    }

    /// Whether this actuation is merely a suggestion and therefore must stay reviewable.
    pub const fn is_suggestion(self) -> bool {
        matches!(self, Self::Suggested)
    }
}

/// One explicit or suggested action that changes the active workset scope.
///
/// Scope widening is always reviewable, so a graph-backed surface can never silently broaden
/// past the active slice. Suggestions may exist, but they are reviewable too.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeChangeAction {
    /// Stable action id inside the packet.
    pub action_id: String,
    /// Whether the action widens or narrows the slice.
    pub direction: ScopeChangeDirection,
    /// Whether the action is an explicit user action or a suggestion.
    pub actuation: ScopeChangeActuation,
    /// Scope id the slice would move to if this action is taken.
    pub target_scope_id: String,
    /// Whether the action is gated behind explicit review before it takes effect.
    ///
    /// Must be `true` for every widen action and every suggestion.
    pub requires_review: bool,
    /// Reviewer-facing summary of the action.
    pub summary: String,
}

impl ScopeChangeAction {
    /// Whether the action must be reviewable to satisfy the no-silent-broadening invariant.
    pub const fn must_be_reviewable(&self) -> bool {
        self.direction.is_widen() || self.actuation.is_suggestion()
    }

    /// Whether the action's review gate satisfies the no-silent-broadening invariant.
    pub const fn review_gate_ok(&self) -> bool {
        !self.must_be_reviewable() || self.requires_review
    }
}

/// The active scope snapshot every M5 code-understanding surface binds to.
///
/// The snapshot is the replay anchor: its [`WorksetScopeSnapshot::snapshot_id`] is recorded on
/// every binding so support export and replay can reconstruct the exact slice the user queried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorksetScopeSnapshot {
    /// Stable snapshot id selections and results are stamped with.
    pub snapshot_id: String,
    /// UTC date the snapshot was taken.
    pub taken_as_of: String,
    /// Canonical workset-scope descriptor this snapshot bounds.
    pub descriptor: WorksetScopeDescriptor,
}

impl WorksetScopeSnapshot {
    /// Whether the active descriptor covers the full workspace.
    pub fn is_full_workspace(&self) -> bool {
        self.descriptor.scope_mode == WorksetScopeMode::Full
    }
}

/// One M5 code-understanding surface bound to the active scope snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeConsumerBinding {
    /// Stable binding id inside the packet.
    pub binding_id: String,
    /// Surface this binding scopes.
    pub surface: WorksetScopeConsumerSurface,
    /// Snapshot id this surface is bound to; must equal the active snapshot id.
    pub snapshot_id: String,
    /// Scope id this surface renders; must equal the active descriptor's scope id.
    pub scope_id: String,
    /// Whether this surface claims whole-workspace knowledge.
    ///
    /// May only be `true` when the active descriptor is [`WorksetScopeMode::Full`].
    pub implies_full_workspace: bool,
    /// Ref to the surface artifact that ingests this scope binding.
    pub consumer_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5WorksetScopeSummary {
    /// Total consumer bindings.
    pub total_bindings: usize,
    /// Number of distinct consumer surfaces claimed.
    pub surface_count: usize,
    /// Total scope-change actions.
    pub total_actions: usize,
    /// Actions that widen the slice.
    pub widen_actions: usize,
    /// Actions that narrow the slice.
    pub narrow_actions: usize,
    /// Actions that are explicit user actions.
    pub explicit_actions: usize,
    /// Actions that are suggestions.
    pub suggested_actions: usize,
    /// Actions gated behind explicit review.
    pub reviewable_actions: usize,
    /// Bindings that claim whole-workspace knowledge.
    pub bindings_implying_full_workspace: usize,
    /// Whether the active scope is sparse rather than full.
    pub active_scope_is_sparse: bool,
    /// Results hidden by the active slice, copied from the descriptor.
    pub hidden_result_count: usize,
    /// Graph objects outside the loaded slice, copied from the descriptor.
    pub not_loaded_count: usize,
}

/// A redaction-safe export row projected from a consumer binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorksetScopeExportRow {
    /// Binding id.
    pub binding_id: String,
    /// Surface token.
    pub surface: String,
    /// Snapshot id this surface is bound to.
    pub snapshot_id: String,
    /// Scope id this surface renders.
    pub scope_id: String,
    /// Whether this surface claims whole-workspace knowledge.
    pub implies_full_workspace: bool,
    /// Surface-artifact ref.
    pub consumer_ref: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the scope index downstream surfaces
/// render instead of restating the active slice by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorksetScopeExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Active snapshot id every binding is stamped with.
    pub snapshot_id: String,
    /// Active scope id.
    pub scope_id: String,
    /// Active scope-mode token.
    pub scope_mode: String,
    /// Active scope-source token.
    pub scope_source: String,
    /// Results hidden by the active slice.
    pub hidden_result_count: usize,
    /// Graph objects outside the loaded slice.
    pub not_loaded_count: usize,
    /// Projected binding rows.
    pub bindings: Vec<M5WorksetScopeExportRow>,
    /// Whether every binding is stamped with the active snapshot id.
    pub all_bindings_snapshot_bound: bool,
    /// Whether any binding claims whole-workspace knowledge over a sparse slice.
    pub any_slice_implies_full_workspace: bool,
    /// Whether every widen action and suggestion is reviewable.
    pub all_widening_reviewable: bool,
}

/// The typed M5 workset-scope descriptor packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5WorksetScopePacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the canonical graph-depth governance matrix this packet extends.
    pub governance_matrix_ref: String,
    /// Ref to the canonical scope-provenance source packet this descriptor draws on.
    pub source_packet_ref: String,
    /// Ref to the graph-conformance suite backing the packet.
    pub conformance_ref: String,
    /// Ref binding this packet into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this packet into the help/service-health surface.
    pub help_surface_ref: String,
    /// Ref binding this packet into the docs-badge surface.
    pub docs_badge_ref: String,
    /// Ref binding this packet into the support-export surface.
    pub support_export_ref: String,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<WorksetScopeConsumerSurface>,
    /// Closed scope-mode vocabulary.
    pub scope_modes: Vec<WorksetScopeMode>,
    /// Closed scope-source vocabulary.
    pub scope_sources: Vec<WorksetScopeSource>,
    /// Closed change-direction vocabulary.
    pub change_directions: Vec<ScopeChangeDirection>,
    /// Closed change-actuation vocabulary.
    pub change_actuations: Vec<ScopeChangeActuation>,
    /// The active scope snapshot every surface binds to.
    pub active_snapshot: WorksetScopeSnapshot,
    /// Explicit and suggested scope-change actions.
    #[serde(default)]
    pub scope_change_actions: Vec<ScopeChangeAction>,
    /// Consumer bindings, one per claimed surface.
    #[serde(default)]
    pub consumer_bindings: Vec<ScopeConsumerBinding>,
    /// Summary counts.
    pub summary: M5WorksetScopeSummary,
}

impl M5WorksetScopePacket {
    /// Returns the binding for a consumer surface.
    pub fn binding(&self, surface: WorksetScopeConsumerSurface) -> Option<&ScopeConsumerBinding> {
        self.consumer_bindings.iter().find(|b| b.surface == surface)
    }

    /// Widen actions offered or suggested by the packet.
    pub fn widen_actions(&self) -> impl Iterator<Item = &ScopeChangeAction> {
        self.scope_change_actions
            .iter()
            .filter(|a| a.direction.is_widen())
    }

    /// Whether every binding is stamped with the active snapshot id.
    pub fn all_bindings_snapshot_bound(&self) -> bool {
        let snapshot_id = &self.active_snapshot.snapshot_id;
        self.consumer_bindings
            .iter()
            .all(|b| &b.snapshot_id == snapshot_id)
    }

    /// Whether any binding claims whole-workspace knowledge while the active scope is sparse.
    ///
    /// This is the guardrail probe: it must always be `false` in a valid packet.
    pub fn any_slice_implies_full_workspace(&self) -> bool {
        !self.active_snapshot.is_full_workspace()
            && self
                .consumer_bindings
                .iter()
                .any(|b| b.implies_full_workspace)
    }

    /// Whether every widen action and every suggestion is reviewable.
    pub fn all_widening_reviewable(&self) -> bool {
        self.scope_change_actions
            .iter()
            .all(ScopeChangeAction::review_gate_ok)
    }

    /// Recomputes the summary block from the snapshot, actions, and bindings.
    pub fn computed_summary(&self) -> M5WorksetScopeSummary {
        let coverage: &IndexCoverage = &self.active_snapshot.descriptor.index_coverage;
        M5WorksetScopeSummary {
            total_bindings: self.consumer_bindings.len(),
            surface_count: self.consumer_surfaces.len(),
            total_actions: self.scope_change_actions.len(),
            widen_actions: self
                .scope_change_actions
                .iter()
                .filter(|a| a.direction == ScopeChangeDirection::Widen)
                .count(),
            narrow_actions: self
                .scope_change_actions
                .iter()
                .filter(|a| a.direction == ScopeChangeDirection::Narrow)
                .count(),
            explicit_actions: self
                .scope_change_actions
                .iter()
                .filter(|a| a.actuation == ScopeChangeActuation::Explicit)
                .count(),
            suggested_actions: self
                .scope_change_actions
                .iter()
                .filter(|a| a.actuation == ScopeChangeActuation::Suggested)
                .count(),
            reviewable_actions: self
                .scope_change_actions
                .iter()
                .filter(|a| a.requires_review)
                .count(),
            bindings_implying_full_workspace: self
                .consumer_bindings
                .iter()
                .filter(|b| b.implies_full_workspace)
                .count(),
            active_scope_is_sparse: self.active_snapshot.descriptor.scope_mode
                == WorksetScopeMode::Sparse,
            hidden_result_count: self.active_snapshot.descriptor.hidden_result_count,
            not_loaded_count: coverage.not_loaded_count,
        }
    }

    /// Produces the scope index downstream surfaces — release evidence,
    /// help/service-health, docs badges, and support exports — render instead of restating
    /// the active slice by hand.
    pub fn export_projection(&self) -> M5WorksetScopeExportProjection {
        let descriptor = &self.active_snapshot.descriptor;
        let bindings = self
            .consumer_bindings
            .iter()
            .map(|b| M5WorksetScopeExportRow {
                binding_id: b.binding_id.clone(),
                surface: b.surface.as_str().to_owned(),
                snapshot_id: b.snapshot_id.clone(),
                scope_id: b.scope_id.clone(),
                implies_full_workspace: b.implies_full_workspace,
                consumer_ref: b.consumer_ref.clone(),
                summary: format!(
                    "{}: scope {} ({}), snapshot {}, full_workspace {}",
                    b.surface.as_str(),
                    b.scope_id,
                    descriptor.scope_mode.as_str(),
                    b.snapshot_id,
                    b.implies_full_workspace
                ),
            })
            .collect();
        M5WorksetScopeExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            snapshot_id: self.active_snapshot.snapshot_id.clone(),
            scope_id: descriptor.scope_id.clone(),
            scope_mode: descriptor.scope_mode.as_str().to_owned(),
            scope_source: descriptor.scope_source.as_str().to_owned(),
            hidden_result_count: descriptor.hidden_result_count,
            not_loaded_count: descriptor.index_coverage.not_loaded_count,
            bindings,
            all_bindings_snapshot_bound: self.all_bindings_snapshot_bound(),
            any_slice_implies_full_workspace: self.any_slice_implies_full_workspace(),
            all_widening_reviewable: self.all_widening_reviewable(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5WorksetScopeViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_snapshot(&mut violations);
        self.validate_actions(&mut violations);
        self.validate_bindings(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5WorksetScopeViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5WorksetScopeViolation>) {
        if self.schema_version != M5_WORKSET_SCOPE_SCHEMA_VERSION {
            violations.push(M5WorksetScopeViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_WORKSET_SCOPE_RECORD_KIND {
            violations.push(M5WorksetScopeViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("governance_matrix_ref", &self.governance_matrix_ref),
            ("source_packet_ref", &self.source_packet_ref),
            ("conformance_ref", &self.conformance_ref),
            ("release_evidence_ref", &self.release_evidence_ref),
            ("help_surface_ref", &self.help_surface_ref),
            ("docs_badge_ref", &self.docs_badge_ref),
            ("support_export_ref", &self.support_export_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5WorksetScopeViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        // The packet must bind upstream to the canonical governance matrix and scope-provenance
        // source packet it extends, so the shared scope model has one provenance root.
        if self.governance_matrix_ref != M5_WORKSET_SCOPE_GOVERNANCE_MATRIX_REF {
            violations.push(M5WorksetScopeViolation::GovernanceMatrixRefMismatch);
        }
        if self.source_packet_ref != M5_WORKSET_SCOPE_SOURCE_PACKET_REF {
            violations.push(M5WorksetScopeViolation::SourcePacketRefMismatch);
        }
        for (field, ok) in [
            (
                "consumer_surfaces",
                self.consumer_surfaces == WorksetScopeConsumerSurface::ALL.to_vec(),
            ),
            (
                "scope_modes",
                self.scope_modes == vec![WorksetScopeMode::Full, WorksetScopeMode::Sparse],
            ),
            (
                "scope_sources",
                self.scope_sources == vec![WorksetScopeSource::Local, WorksetScopeSource::Managed],
            ),
            (
                "change_directions",
                self.change_directions == ScopeChangeDirection::ALL.to_vec(),
            ),
            (
                "change_actuations",
                self.change_actuations == ScopeChangeActuation::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5WorksetScopeViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_snapshot(&self, violations: &mut Vec<M5WorksetScopeViolation>) {
        let snapshot = &self.active_snapshot;
        let descriptor = &snapshot.descriptor;
        for (field, value) in [
            ("snapshot_id", &snapshot.snapshot_id),
            ("taken_as_of", &snapshot.taken_as_of),
            ("scope_id", &descriptor.scope_id),
            ("scope_class", &descriptor.scope_class),
            ("coverage_state", &descriptor.index_coverage.coverage_state),
        ] {
            if value.trim().is_empty() {
                violations.push(M5WorksetScopeViolation::EmptyField {
                    id: snapshot.snapshot_id.clone(),
                    field_name: field,
                });
            }
        }
        // A scope descriptor that names no included root or repo cannot tell the user what
        // slice it bounds.
        if descriptor.included_roots_or_repos.is_empty() {
            violations.push(M5WorksetScopeViolation::EmptyField {
                id: snapshot.snapshot_id.clone(),
                field_name: "included_roots_or_repos",
            });
        }
        // A full-workspace scope hides and unloads nothing; a non-zero hidden or not-loaded
        // count would be a slice masquerading as the whole workspace.
        if descriptor.scope_mode == WorksetScopeMode::Full
            && (descriptor.hidden_result_count != 0
                || descriptor.index_coverage.not_loaded_count != 0)
        {
            violations.push(M5WorksetScopeViolation::FullScopeHidesResults {
                snapshot_id: snapshot.snapshot_id.clone(),
            });
        }
    }

    fn validate_actions(&self, violations: &mut Vec<M5WorksetScopeViolation>) {
        let mut seen_ids = BTreeSet::new();
        for action in &self.scope_change_actions {
            if !seen_ids.insert(action.action_id.clone()) {
                violations.push(M5WorksetScopeViolation::DuplicateActionId {
                    action_id: action.action_id.clone(),
                });
            }
            for (field, value) in [
                ("action_id", &action.action_id),
                ("target_scope_id", &action.target_scope_id),
                ("summary", &action.summary),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5WorksetScopeViolation::EmptyField {
                        id: action.action_id.clone(),
                        field_name: field,
                    });
                }
            }
            // No silent broadening: a widen action or a suggestion must be reviewable.
            if !action.review_gate_ok() {
                violations.push(M5WorksetScopeViolation::SilentBroadening {
                    action_id: action.action_id.clone(),
                });
            }
        }
        // The packet must offer at least one explicit widen action so scope widening happens
        // through an explicit, reviewable action rather than silent background broadening.
        if !self.scope_change_actions.iter().any(|a| {
            a.direction == ScopeChangeDirection::Widen
                && a.actuation == ScopeChangeActuation::Explicit
        }) {
            violations.push(M5WorksetScopeViolation::MissingExplicitWidenAction);
        }
    }

    fn validate_bindings(&self, violations: &mut Vec<M5WorksetScopeViolation>) {
        let snapshot_id = &self.active_snapshot.snapshot_id;
        let scope_id = &self.active_snapshot.descriptor.scope_id;
        let is_full = self.active_snapshot.is_full_workspace();

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for binding in &self.consumer_bindings {
            if !seen_ids.insert(binding.binding_id.clone()) {
                violations.push(M5WorksetScopeViolation::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if !seen_surfaces.insert(binding.surface) {
                violations.push(M5WorksetScopeViolation::DuplicateSurfaceBinding {
                    surface: binding.surface.as_str(),
                });
            }
            for (field, value) in [
                ("binding_id", &binding.binding_id),
                ("snapshot_id", &binding.snapshot_id),
                ("scope_id", &binding.scope_id),
                ("consumer_ref", &binding.consumer_ref),
                ("note", &binding.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5WorksetScopeViolation::EmptyField {
                        id: binding.binding_id.clone(),
                        field_name: field,
                    });
                }
            }
            // Every binding must be stamped with the active snapshot so support export and
            // replay can reconstruct the slice the user queried.
            if &binding.snapshot_id != snapshot_id {
                violations.push(M5WorksetScopeViolation::SnapshotBindingMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if &binding.scope_id != scope_id {
                violations.push(M5WorksetScopeViolation::ScopeIdMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            // A binding may only claim whole-workspace knowledge when the active descriptor
            // covers the full workspace, so a slice never masquerades as the whole workspace.
            if binding.implies_full_workspace && !is_full {
                violations.push(M5WorksetScopeViolation::FullWorkspaceClaimOverSlice {
                    binding_id: binding.binding_id.clone(),
                });
            }
        }

        // Every consumer surface must carry a binding so no surface leaves its scope boundary
        // implicit.
        for surface in WorksetScopeConsumerSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5WorksetScopeViolation::MissingSurfaceBinding {
                    surface: surface.as_str(),
                });
            }
        }
    }
}

/// A validation violation for the M5 workset-scope packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WorksetScopeViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The packet does not bind to the canonical governance matrix.
    GovernanceMatrixRefMismatch,
    /// The packet does not bind to the canonical scope-provenance source packet.
    SourcePacketRefMismatch,
    /// A full-workspace scope reports hidden or not-loaded results.
    FullScopeHidesResults {
        /// Snapshot id.
        snapshot_id: String,
    },
    /// A scope-change action id appears more than once.
    DuplicateActionId {
        /// Duplicate action id.
        action_id: String,
    },
    /// A widen action or suggestion is not reviewable.
    SilentBroadening {
        /// Action id.
        action_id: String,
    },
    /// The packet offers no explicit widen action.
    MissingExplicitWidenAction,
    /// A binding id appears more than once.
    DuplicateBindingId {
        /// Duplicate binding id.
        binding_id: String,
    },
    /// A consumer surface carries more than one binding.
    DuplicateSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer surface has no binding.
    MissingSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A binding is not stamped with the active snapshot id.
    SnapshotBindingMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding renders a scope id other than the active descriptor's.
    ScopeIdMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding claims whole-workspace knowledge while bound to a sparse slice.
    FullWorkspaceClaimOverSlice {
        /// Binding id.
        binding_id: String,
    },
    /// The summary counts disagree with the snapshot, actions, and bindings.
    SummaryMismatch,
}

impl fmt::Display for M5WorksetScopeViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::GovernanceMatrixRefMismatch => {
                write!(
                    f,
                    "packet governance_matrix_ref must be the canonical graph-depth governance matrix"
                )
            }
            Self::SourcePacketRefMismatch => {
                write!(
                    f,
                    "packet source_packet_ref must be the canonical scope-provenance truth packet"
                )
            }
            Self::FullScopeHidesResults { snapshot_id } => {
                write!(
                    f,
                    "snapshot {snapshot_id} is full-workspace but reports hidden or not-loaded results"
                )
            }
            Self::DuplicateActionId { action_id } => {
                write!(f, "duplicate scope-change action id {action_id}")
            }
            Self::SilentBroadening { action_id } => {
                write!(
                    f,
                    "action {action_id} widens or suggests scope but is not reviewable"
                )
            }
            Self::MissingExplicitWidenAction => {
                write!(f, "packet offers no explicit widen action")
            }
            Self::DuplicateBindingId { binding_id } => {
                write!(f, "duplicate binding id {binding_id}")
            }
            Self::DuplicateSurfaceBinding { surface } => {
                write!(f, "duplicate binding for surface {surface}")
            }
            Self::MissingSurfaceBinding { surface } => {
                write!(f, "missing binding for surface {surface}")
            }
            Self::SnapshotBindingMismatch { binding_id } => {
                write!(
                    f,
                    "binding {binding_id} is not stamped with the active snapshot id"
                )
            }
            Self::ScopeIdMismatch { binding_id } => {
                write!(
                    f,
                    "binding {binding_id} renders a scope other than the active descriptor's"
                )
            }
            Self::FullWorkspaceClaimOverSlice { binding_id } => {
                write!(
                    f,
                    "binding {binding_id} claims whole-workspace knowledge over a sparse slice"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the packet body")
            }
        }
    }
}

impl Error for M5WorksetScopeViolation {}

/// Loads the embedded M5 workset-scope packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5WorksetScopePacket`].
pub fn current_m5_workset_scope_packet() -> Result<M5WorksetScopePacket, serde_json::Error> {
    serde_json::from_str(M5_WORKSET_SCOPE_JSON)
}

#[cfg(test)]
mod tests;
