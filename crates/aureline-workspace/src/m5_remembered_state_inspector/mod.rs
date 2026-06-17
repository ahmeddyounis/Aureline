//! The M5 remembered-state inspector: one inspectable surface that lists what the current
//! windows and workspace will remember, when it was last written, how truthfully it restores,
//! whether it is portable, and what can be compared, exported, or cleared — without reading logs
//! or raw JSON.
//!
//! The serialization-and-restore matrix classifies *what* M5 may remember and the
//! remembered-state objects *implement* the underlying state. This packet is the **user- and
//! operator-facing projection** of both: it never invents its own restore or ownership language.
//! Every row reuses the canonical [`RememberedArtifactClass`], [`OwnershipClass`], and
//! [`RestoreFidelityClass`] vocabularies from
//! [`crate::m5_serialization_and_restore_matrix`], so remembered-state meaning cannot fork by
//! surface.
//!
//! - [`InspectorRow`] exposes, per remembered-state class relevant to the current
//!   windows/workspace: the artifact class, a plain-language title, the canonical state-object
//!   reference, the last-write time, the schema version, producer/build provenance, a
//!   portable/shared/local/machine-local label, and what the class *will* and *will not* remember.
//! - [`ActionAffordance`] models the inspect, export, compare, and clear actions. Every affordance
//!   is bounded to the one selected state class ([`ActionBoundary::SelectedStateClassOnly`]),
//!   excludes unrelated content and caches, carries a command id, a keyboard shortcut, a
//!   deterministic focus order, and a screen-reader label — so the flows are keyboard-complete and
//!   screen-reader-safe. [`ActionBoundary::GlobalReset`] is reject-only: it exists in the
//!   vocabulary so the gate can refuse a clear that would look like a destructive global reset.
//! - [`ConsumerBinding`] wires the four reuse surfaces — diagnostics, crash recovery,
//!   browser/companion handoff, and support export — to this one packet so they render the same
//!   labels instead of re-deriving them.
//!
//! The packet is fail-closed. [`M5RememberedStateInspector::validate`] rejects a row whose
//! `exportable` flag disagrees with its ownership, a non-exportable class that offers export, a
//! row that hides its meaning by omitting the inspect action, an inaccessible affordance, a clear
//! that is not bounded or not confirmed, and a clear modeled as a global reset. A non-exportable
//! row never offers export, and local-only and machine-local state stays visible but unexportable.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-remembered-state-inspector.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, an opaque ref, or a
//! plain-language label, and it carries no credential bodies, raw provider payloads, live authority
//! handles, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_serialization_and_restore_matrix::{
    OwnershipClass, RememberedArtifactClass, RestoreFidelityClass,
};

/// Supported M5 remembered-state inspector packet schema version.
pub const M5_REMEMBERED_STATE_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_REMEMBERED_STATE_INSPECTOR_RECORD_KIND: &str = "m5_remembered_state_inspector";

/// Repo-relative path to the checked-in packet.
pub const M5_REMEMBERED_STATE_INSPECTOR_PATH: &str =
    "artifacts/workspace/m5/m5-remembered-state-inspector.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_REMEMBERED_STATE_INSPECTOR_SCHEMA_REF: &str =
    "schemas/workspace/m5-remembered-state-inspector.schema.json";

/// Repo-relative path to the companion document.
pub const M5_REMEMBERED_STATE_INSPECTOR_DOC_REF: &str =
    "docs/workspace/m5/m5-remembered-state-inspector.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_REMEMBERED_STATE_INSPECTOR_ARTIFACT_DOC_REF: &str =
    "artifacts/workspace/m5/m5-remembered-state-inspector.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_REMEMBERED_STATE_INSPECTOR_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-remembered-state-inspector";

/// Embedded checked-in packet JSON.
pub const M5_REMEMBERED_STATE_INSPECTOR_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-remembered-state-inspector.json"
));

// --- Action vocabulary ---------------------------------------------------------------------------

/// One of the four actions the inspector offers on a remembered-state class.
///
/// Inspect and compare are read-only; export copies portable state out without mutating it; clear
/// is the only action that removes remembered state, and it is bounded to the one selected class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorActionKind {
    /// Read the remembered state in plain language; never mutates and never requires a flag.
    Inspect,
    /// Export the remembered state into a portable-state package; only offered for exportable state.
    Export,
    /// Compare two remembered states of the same class for review before reuse.
    Compare,
    /// Clear the remembered state for the selected class only; bounded, confirmed, never global.
    Clear,
}

impl InspectorActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Inspect, Self::Export, Self::Compare, Self::Clear];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Export => "export",
            Self::Compare => "compare",
            Self::Clear => "clear",
        }
    }

    /// Whether the action removes remembered state. Only [`Self::Clear`] mutates.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Clear)
    }
}

/// How far an action is allowed to reach.
///
/// The inspector permits only [`ActionBoundary::SelectedStateClassOnly`].
/// [`ActionBoundary::GlobalReset`] is reject-only: it exists in the vocabulary so the gate can
/// refuse a clear that would silently remove unrelated workspace content or look like a destructive
/// global reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionBoundary {
    /// The action touches only the one remembered-state class the row governs.
    SelectedStateClassOnly,
    /// The action would reset state beyond the selected class. **Forbidden** — reject-only.
    GlobalReset,
}

impl ActionBoundary {
    /// Every boundary, in declaration order.
    pub const ALL: [Self; 2] = [Self::SelectedStateClassOnly, Self::GlobalReset];

    /// The boundaries the inspector permits; [`Self::GlobalReset`] is never one of them.
    pub const ALLOWED: [Self; 1] = [Self::SelectedStateClassOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedStateClassOnly => "selected_state_class_only",
            Self::GlobalReset => "global_reset",
        }
    }

    /// Whether the boundary keeps the action bounded to the selected class.
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::SelectedStateClassOnly)
    }
}

/// A surface that must reuse the inspector's remembered-state vocabulary rather than inventing
/// its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorConsumerSurface {
    /// The workspace diagnostics surface.
    Diagnostics,
    /// Crash recovery and unsaved-state restore.
    CrashRecovery,
    /// Browser and mobile companion handoff.
    BrowserCompanionHandoff,
    /// The support-export bundle.
    SupportExport,
}

impl InspectorConsumerSurface {
    /// Every consumer surface that must reuse this packet, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::Diagnostics,
        Self::CrashRecovery,
        Self::BrowserCompanionHandoff,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diagnostics => "diagnostics",
            Self::CrashRecovery => "crash_recovery",
            Self::BrowserCompanionHandoff => "browser_companion_handoff",
            Self::SupportExport => "support_export",
        }
    }
}

// --- Provenance and affordances ------------------------------------------------------------------

/// Producer and build provenance for a remembered-state class, in opaque refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProducerProvenance {
    /// Opaque reference to the component that produced the remembered state.
    pub producer_ref: String,
    /// Opaque reference to the build that wrote it.
    pub build_ref: String,
}

impl ProducerProvenance {
    /// Whether both provenance refs are present.
    pub fn is_complete(&self) -> bool {
        !self.producer_ref.trim().is_empty() && !self.build_ref.trim().is_empty()
    }
}

/// A keyboard-complete, screen-reader-safe affordance for one inspector action.
///
/// Every affordance carries a command id (so it is reachable from the command palette), a keyboard
/// shortcut, a deterministic focus order (so keyboard navigation is unambiguous), and a
/// screen-reader label. It is bounded to the selected class and excludes unrelated content and
/// caches, so no action — least of all clear — can silently widen its blast radius.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionAffordance {
    /// Which action this affordance offers.
    pub action: InspectorActionKind,
    /// Opaque command id; the action is reachable from the command palette by this id.
    pub command_id: String,
    /// Keyboard shortcut token (e.g. `mod+alt+i`); the flow is operable without a pointer.
    pub keyboard_shortcut: String,
    /// Deterministic focus order within the row, so keyboard navigation is unambiguous.
    pub focus_order: u32,
    /// Screen-reader label naming the action and the class it touches.
    pub accessible_label: String,
    /// How far the action reaches. Must be [`ActionBoundary::SelectedStateClassOnly`].
    pub boundary: ActionBoundary,
    /// Whether the action prompts for confirmation. A clear must confirm.
    pub requires_confirmation: bool,
    /// Attestation that the action never touches content outside the selected class. Must be true.
    pub excludes_unrelated_content: bool,
    /// Attestation that the action never clears unrelated caches. Must be true.
    pub excludes_caches: bool,
}

impl ActionAffordance {
    /// Whether the affordance is keyboard-complete and screen-reader-safe.
    pub fn is_accessible(&self) -> bool {
        !self.command_id.trim().is_empty()
            && !self.keyboard_shortcut.trim().is_empty()
            && !self.accessible_label.trim().is_empty()
    }

    /// Whether the affordance stays bounded and touches no unrelated content or caches.
    pub fn is_bounded(&self) -> bool {
        self.boundary.is_bounded() && self.excludes_unrelated_content && self.excludes_caches
    }

    /// Whether a clear affordance is safe: bounded and confirmed, never a silent global reset.
    pub fn is_safe_clear(&self) -> bool {
        self.action != InspectorActionKind::Clear
            || (self.is_bounded() && self.requires_confirmation)
    }
}

// --- Inspector row -------------------------------------------------------------------------------

/// One inspector row: a remembered-state class relevant to the current windows/workspace, with its
/// labels, provenance, and bounded actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectorRow {
    /// Stable row id.
    pub row_id: String,
    /// Remembered-state artifact class this row inspects. Reused from the matrix vocabulary.
    pub artifact_class: RememberedArtifactClass,
    /// Plain-language title shown in the inspector.
    pub title: String,
    /// Opaque reference to the canonical state object this row inspects.
    pub state_object_ref: String,
    /// Opaque last-write time token; never a wall-clock secret or a raw path.
    pub last_write: String,
    /// Schema version of the underlying remembered-state object.
    pub schema_version: u32,
    /// Producer and build provenance.
    pub provenance: ProducerProvenance,
    /// Portable/shared/local/machine-local ownership label. Reused from the matrix vocabulary.
    pub ownership: OwnershipClass,
    /// Whether the class may be exported into a portable-state package. Must follow `ownership`.
    pub exportable: bool,
    /// How truthfully the class restores. Reused from the matrix vocabulary.
    pub published_fidelity: RestoreFidelityClass,
    /// Plain words: what this class will remember.
    pub what_is_remembered: String,
    /// Plain words: what this class will not remember.
    pub what_is_not_remembered: String,
    /// Bounded actions offered on the class.
    #[serde(default)]
    pub available_actions: Vec<ActionAffordance>,
    /// Reviewer-facing caveats attached to the row.
    #[serde(default)]
    pub caveats: Vec<String>,
}

impl InspectorRow {
    /// The `exportable` value the row's ownership implies.
    pub fn expected_exportable(&self) -> bool {
        self.ownership.exportable_into_portable_package()
    }

    /// Whether the class is portable or shared (and therefore exportable).
    pub fn is_portable(&self) -> bool {
        self.ownership.exportable_into_portable_package()
    }

    /// The affordance for an action, if the row offers it.
    pub fn action(&self, kind: InspectorActionKind) -> Option<&ActionAffordance> {
        self.available_actions.iter().find(|a| a.action == kind)
    }

    /// Whether the row offers an action.
    pub fn has_action(&self, kind: InspectorActionKind) -> bool {
        self.action(kind).is_some()
    }

    /// Whether the row can be cleared (offers a bounded, confirmed clear).
    pub fn is_clearable(&self) -> bool {
        self.action(InspectorActionKind::Clear)
            .is_some_and(ActionAffordance::is_safe_clear)
    }

    /// A plain-language summary line for the inspect view.
    fn summary_line(&self) -> String {
        let visibility = if self.is_portable() {
            "exportable"
        } else {
            "local-only"
        };
        format!(
            "{}: {} ({}), schema v{}, restores {}, last written {}",
            self.artifact_class.as_str(),
            self.ownership.as_str(),
            visibility,
            self.schema_version,
            self.published_fidelity.as_str(),
            self.last_write
        )
    }
}

// --- Consumer binding ----------------------------------------------------------------------------

/// A binding wiring a reuse surface to this inspector packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsumerBinding {
    /// Surface this binding wires.
    pub consumer_surface: InspectorConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Inspector packet id this surface ingests.
    pub inspector_packet_id_ref: String,
    /// True when the surface reuses this packet's vocabulary rather than inventing its own.
    pub reuses_inspector_vocabulary: bool,
    /// True when the surface preserves the ownership/portability labels verbatim.
    pub preserves_ownership_labels: bool,
    /// True when the surface preserves the restore-fidelity labels verbatim.
    pub preserves_fidelity_labels: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl ConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.inspector_packet_id_ref == packet_id
            && self.reuses_inspector_vocabulary
            && self.preserves_ownership_labels
            && self.preserves_fidelity_labels
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

// --- Summary and views ---------------------------------------------------------------------------

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RememberedStateInspectorSummary {
    /// Total inspector rows.
    pub rows: usize,
    /// Rows whose state is exportable (portable or shared).
    pub exportable_rows: usize,
    /// Rows whose state is local-only or machine-local (never exported).
    pub local_only_rows: usize,
    /// Rows offering a bounded, confirmed clear.
    pub clearable_rows: usize,
    /// Rows offering a compare action.
    pub comparable_rows: usize,
}

/// A plain-language inspect-view row — what the inspector renders so a user never reads raw JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorViewRow {
    /// Artifact-class token.
    pub artifact_class: String,
    /// Plain-language title.
    pub title: String,
    /// Ownership/portability token.
    pub ownership: String,
    /// Whether the class is exportable.
    pub exportable: bool,
    /// Schema version of the underlying object.
    pub schema_version: u32,
    /// Last-write time token.
    pub last_write: String,
    /// Published restore-fidelity token.
    pub published_fidelity: String,
    /// Plain words: what the class will remember.
    pub what_is_remembered: String,
    /// Plain words: what the class will not remember.
    pub what_is_not_remembered: String,
    /// Action tokens offered on the class.
    pub actions: Vec<String>,
    /// Human-readable summary line.
    pub summary: String,
}

/// The plain-language inspect view downstream surfaces render instead of restating each row by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorView {
    /// Packet id this view was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<InspectorViewRow>,
    /// Rows whose state is exportable.
    pub exportable_count: usize,
    /// Rows offering a bounded clear.
    pub clearable_count: usize,
}

// --- Packet --------------------------------------------------------------------------------------

/// The typed M5 remembered-state inspector packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RememberedStateInspector {
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
    /// Closed remembered-state artifact-class vocabulary.
    pub artifact_classes: Vec<RememberedArtifactClass>,
    /// Closed ownership-class vocabulary.
    pub ownership_classes: Vec<OwnershipClass>,
    /// Closed restore-fidelity-class vocabulary.
    pub restore_fidelity_classes: Vec<RestoreFidelityClass>,
    /// Closed action-kind vocabulary.
    pub action_kinds: Vec<InspectorActionKind>,
    /// Closed action-boundary vocabulary.
    pub action_boundaries: Vec<ActionBoundary>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<InspectorConsumerSurface>,
    /// Inspector rows, one per remembered-state class relevant to the current windows/workspace.
    #[serde(default)]
    pub rows: Vec<InspectorRow>,
    /// Consumer bindings, one per required reuse surface.
    #[serde(default)]
    pub consumer_bindings: Vec<ConsumerBinding>,
    /// Summary counts.
    pub summary: M5RememberedStateInspectorSummary,
}

impl M5RememberedStateInspector {
    /// Returns the row for a remembered-state artifact class.
    pub fn row(&self, class: RememberedArtifactClass) -> Option<&InspectorRow> {
        self.rows.iter().find(|r| r.artifact_class == class)
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, surface: InspectorConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5RememberedStateInspectorSummary {
        M5RememberedStateInspectorSummary {
            rows: self.rows.len(),
            exportable_rows: self.rows.iter().filter(|r| r.exportable).count(),
            local_only_rows: self.rows.iter().filter(|r| !r.exportable).count(),
            clearable_rows: self.rows.iter().filter(|r| r.is_clearable()).count(),
            comparable_rows: self
                .rows
                .iter()
                .filter(|r| r.has_action(InspectorActionKind::Compare))
                .count(),
        }
    }

    /// Produces the plain-language inspect view downstream surfaces render.
    pub fn inspect_view(&self) -> InspectorView {
        let rows = self
            .rows
            .iter()
            .map(|r| InspectorViewRow {
                artifact_class: r.artifact_class.as_str().to_owned(),
                title: r.title.clone(),
                ownership: r.ownership.as_str().to_owned(),
                exportable: r.exportable,
                schema_version: r.schema_version,
                last_write: r.last_write.clone(),
                published_fidelity: r.published_fidelity.as_str().to_owned(),
                what_is_remembered: r.what_is_remembered.clone(),
                what_is_not_remembered: r.what_is_not_remembered.clone(),
                actions: r
                    .available_actions
                    .iter()
                    .map(|a| a.action.as_str().to_owned())
                    .collect(),
                summary: r.summary_line(),
            })
            .collect();
        InspectorView {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            exportable_count: self.rows.iter().filter(|r| r.exportable).count(),
            clearable_count: self.rows.iter().filter(|r| r.is_clearable()).count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact inspector.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5RememberedStateInspectorSupportExport {
        M5RememberedStateInspectorSupportExport {
            record_kind: M5_REMEMBERED_STATE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_REMEMBERED_STATE_INSPECTOR_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            packet: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5RememberedStateInspectorViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5RememberedStateInspectorViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_classes.insert(row.artifact_class) {
                violations.push(
                    M5RememberedStateInspectorViolation::DuplicateArtifactClassRow {
                        class: row.artifact_class.as_str(),
                    },
                );
            }
            self.validate_row(row, &mut violations);
        }
        for &class in &RememberedArtifactClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(
                    M5RememberedStateInspectorViolation::MissingArtifactClassRow {
                        class: class.as_str(),
                    },
                );
            }
        }

        for surface in InspectorConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(
                    M5RememberedStateInspectorViolation::MissingConsumerBinding {
                        surface: surface.as_str(),
                    },
                );
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5RememberedStateInspectorViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5RememberedStateInspectorViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5RememberedStateInspectorViolation>) {
        if self.schema_version != M5_REMEMBERED_STATE_INSPECTOR_SCHEMA_VERSION {
            violations.push(
                M5RememberedStateInspectorViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_REMEMBERED_STATE_INSPECTOR_RECORD_KIND {
            violations.push(M5RememberedStateInspectorViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RememberedStateInspectorViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_classes",
                self.artifact_classes == RememberedArtifactClass::ALL.to_vec(),
            ),
            (
                "ownership_classes",
                self.ownership_classes == OwnershipClass::ALL.to_vec(),
            ),
            (
                "restore_fidelity_classes",
                self.restore_fidelity_classes == RestoreFidelityClass::ALL.to_vec(),
            ),
            (
                "action_kinds",
                self.action_kinds == InspectorActionKind::ALL.to_vec(),
            ),
            (
                "action_boundaries",
                self.action_boundaries == ActionBoundary::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == InspectorConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5RememberedStateInspectorViolation::ClosedVocabularyDrift {
                    field_name: field,
                });
            }
        }
    }

    fn validate_row(
        &self,
        row: &InspectorRow,
        violations: &mut Vec<M5RememberedStateInspectorViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("title", &row.title),
            ("state_object_ref", &row.state_object_ref),
            ("last_write", &row.last_write),
            ("what_is_remembered", &row.what_is_remembered),
            ("what_is_not_remembered", &row.what_is_not_remembered),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RememberedStateInspectorViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }
        if row.schema_version == 0 {
            violations.push(M5RememberedStateInspectorViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "schema_version",
            });
        }
        if !row.provenance.is_complete() {
            violations.push(M5RememberedStateInspectorViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "provenance",
            });
        }

        // Portable/local/shared/machine-local must agree with exportability: portable and shared
        // state is exportable; local-only and machine-local state is never exported.
        if row.exportable != row.expected_exportable() {
            violations.push(M5RememberedStateInspectorViolation::ExportabilityMismatch {
                row_id: row.row_id.clone(),
                ownership: row.ownership.as_str(),
                exportable: row.exportable,
            });
        }

        // The inspector must never hide a class's meaning behind a debug flag or a raw dump: every
        // row offers a read-only inspect action.
        if !row.has_action(InspectorActionKind::Inspect) {
            violations.push(M5RememberedStateInspectorViolation::MissingInspectAction {
                row_id: row.row_id.clone(),
            });
        }

        // Export is offered exactly when the class is exportable. A non-exportable class never
        // offers export; an exportable class must.
        match (row.exportable, row.has_action(InspectorActionKind::Export)) {
            (false, true) => {
                violations.push(
                    M5RememberedStateInspectorViolation::NonExportableOffersExport {
                        row_id: row.row_id.clone(),
                        ownership: row.ownership.as_str(),
                    },
                );
            }
            (true, false) => {
                violations.push(
                    M5RememberedStateInspectorViolation::ExportableMissingExport {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            _ => {}
        }

        let mut seen_actions = BTreeSet::new();
        let mut seen_focus = BTreeSet::new();
        for affordance in &row.available_actions {
            if !seen_actions.insert(affordance.action) {
                violations.push(M5RememberedStateInspectorViolation::DuplicateAction {
                    row_id: row.row_id.clone(),
                    action: affordance.action.as_str(),
                });
            }
            if !seen_focus.insert(affordance.focus_order) {
                violations.push(M5RememberedStateInspectorViolation::DuplicateFocusOrder {
                    row_id: row.row_id.clone(),
                });
            }
            if !affordance.is_accessible() {
                violations.push(
                    M5RememberedStateInspectorViolation::InaccessibleAffordance {
                        row_id: row.row_id.clone(),
                        action: affordance.action.as_str(),
                    },
                );
            }
            // Every action stays bounded to the selected class and touches no unrelated content or
            // caches; a global-reset boundary is the reject-only case.
            if !affordance.is_bounded() {
                violations.push(M5RememberedStateInspectorViolation::UnboundedAction {
                    row_id: row.row_id.clone(),
                    action: affordance.action.as_str(),
                    boundary: affordance.boundary.as_str(),
                });
            }
            // A clear must be bounded and confirmed so it never looks like a destructive global
            // reset or silently removes unrelated content.
            if !affordance.is_safe_clear() {
                violations.push(M5RememberedStateInspectorViolation::UnsafeClear {
                    row_id: row.row_id.clone(),
                });
            }
        }
    }
}

/// A validation violation for [`M5RememberedStateInspector`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum M5RememberedStateInspectorViolation {
    /// The packet schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Observed version.
        actual: u32,
    },
    /// The packet record kind is unsupported.
    UnsupportedRecordKind {
        /// Observed record kind.
        actual: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Owning object id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A closed-vocabulary array disagrees with this build's canonical list.
    ClosedVocabularyDrift {
        /// Field name.
        field_name: &'static str,
    },
    /// Two rows share a row id.
    DuplicateRowId {
        /// Offending row id.
        row_id: String,
    },
    /// Two rows govern the same artifact class.
    DuplicateArtifactClassRow {
        /// Offending class token.
        class: &'static str,
    },
    /// A remembered-state artifact class has no inspector row.
    MissingArtifactClassRow {
        /// Missing class token.
        class: &'static str,
    },
    /// A row's `exportable` flag disagrees with its ownership.
    ExportabilityMismatch {
        /// Row id.
        row_id: String,
        /// Ownership token.
        ownership: &'static str,
        /// Observed exportable flag.
        exportable: bool,
    },
    /// A row omits the read-only inspect action, hiding its meaning.
    MissingInspectAction {
        /// Row id.
        row_id: String,
    },
    /// A non-exportable class offers an export action.
    NonExportableOffersExport {
        /// Row id.
        row_id: String,
        /// Ownership token.
        ownership: &'static str,
    },
    /// An exportable class omits its export action.
    ExportableMissingExport {
        /// Row id.
        row_id: String,
    },
    /// A row offers the same action twice.
    DuplicateAction {
        /// Row id.
        row_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// Two affordances in a row share a focus order, making keyboard navigation ambiguous.
    DuplicateFocusOrder {
        /// Row id.
        row_id: String,
    },
    /// An affordance lacks a command id, keyboard shortcut, or screen-reader label.
    InaccessibleAffordance {
        /// Row id.
        row_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// An action reaches beyond the selected class or touches unrelated content/caches.
    UnboundedAction {
        /// Row id.
        row_id: String,
        /// Offending action token.
        action: &'static str,
        /// Offending boundary token.
        boundary: &'static str,
    },
    /// A clear is unbounded or unconfirmed and would look like a destructive global reset.
    UnsafeClear {
        /// Row id.
        row_id: String,
    },
    /// A required reuse surface has no preserving consumer binding.
    MissingConsumerBinding {
        /// Missing surface token.
        surface: &'static str,
    },
    /// A consumer binding does not preserve this packet's labels.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5RememberedStateInspectorViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::ClosedVocabularyDrift { field_name } => {
                write!(
                    f,
                    "closed vocabulary {field_name} disagrees with this build"
                )
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate row id {row_id}"),
            Self::DuplicateArtifactClassRow { class } => {
                write!(f, "duplicate inspector row for artifact class {class}")
            }
            Self::MissingArtifactClassRow { class } => {
                write!(f, "artifact class {class} has no inspector row")
            }
            Self::ExportabilityMismatch {
                row_id,
                ownership,
                exportable,
            } => write!(
                f,
                "row {row_id} ownership {ownership} disagrees with exportable={exportable}"
            ),
            Self::MissingInspectAction { row_id } => {
                write!(f, "row {row_id} omits the inspect action")
            }
            Self::NonExportableOffersExport { row_id, ownership } => write!(
                f,
                "row {row_id} ({ownership}) offers export for non-exportable state"
            ),
            Self::ExportableMissingExport { row_id } => {
                write!(f, "row {row_id} is exportable but omits the export action")
            }
            Self::DuplicateAction { row_id, action } => {
                write!(f, "row {row_id} offers action {action} twice")
            }
            Self::DuplicateFocusOrder { row_id } => {
                write!(f, "row {row_id} has affordances sharing a focus order")
            }
            Self::InaccessibleAffordance { row_id, action } => write!(
                f,
                "row {row_id} action {action} lacks a command id, shortcut, or label"
            ),
            Self::UnboundedAction {
                row_id,
                action,
                boundary,
            } => write!(f, "row {row_id} action {action} is unbounded ({boundary})"),
            Self::UnsafeClear { row_id } => write!(
                f,
                "row {row_id} clear is unbounded or unconfirmed and looks like a global reset"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "reuse surface {surface} has no consumer binding")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(
                    f,
                    "consumer binding {binding_ref} does not preserve the packet"
                )
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for M5RememberedStateInspectorViolation {}

/// Stable record-kind tag for [`M5RememberedStateInspectorSupportExport`].
pub const M5_REMEMBERED_STATE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_remembered_state_inspector_support_export";

/// Support-export wrapper preserving the packet verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RememberedStateInspectorSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact packet preserved by the export.
    pub packet: M5RememberedStateInspector,
}

impl M5RememberedStateInspectorSupportExport {
    /// Whether the export preserves the same packet id and a clean packet.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_REMEMBERED_STATE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_REMEMBERED_STATE_INSPECTOR_SCHEMA_VERSION
            && self.packet_id_ref == self.packet.packet_id
            && self.raw_private_material_excluded
            && self.packet.validate().is_empty()
    }
}

/// Loads the embedded M5 remembered-state inspector packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5RememberedStateInspector`].
pub fn current_m5_remembered_state_inspector(
) -> Result<M5RememberedStateInspector, serde_json::Error> {
    serde_json::from_str(M5_REMEMBERED_STATE_INSPECTOR_JSON)
}

#[cfg(test)]
mod tests;
