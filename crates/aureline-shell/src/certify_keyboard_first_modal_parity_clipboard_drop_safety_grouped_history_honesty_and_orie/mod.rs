//! Release-bearing certification of keyboard-first modal parity, clipboard/drop
//! safety, grouped-history honesty, and orientation-aid continuity on every
//! claimed M5 switching / power-user surface row.
//!
//! Where
//! [`crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix`]
//! freezes *which* canonical interaction vocabulary each claimed M5 surface
//! resolves to — and the per-axis consumers
//! ([`crate::mode_strip_leader_sequence_register_picker_and_capability_gap_banner_surfaces`],
//! [`crate::implement_clipboard_contracts_with_plain_text_default_copy_with_context_variants_sensitive`],
//! [`crate::add_drag_and_drop_verb_disclosure_insertion_indicators_cross_window_detach_and_long_transf`],
//! [`crate::ship_named_undo_groups_exact_versus_compensating_recovery_labels_back_forward_history_cont`],
//! [`crate::ship_macro_replay_review_run_capable_or_cross_file_macro_downgrades_and_recipe_promotion`],
//! and
//! [`crate::implement_multi_cursor_fold_state_breadcrumb_minimap_overview_ruler_and_degraded_orientati`])
//! discharge each behavior — this module is the capstone gate. It certifies
//! whether each claimed M5 **row** — editor core, notebook, data/API, preview,
//! docs, review, runtime, or companion-adjacent surface — actually carries
//! *current* proof for every interaction-safety dimension it claims. A row may
//! only keep its parity grade if its modal-keyboard, clipboard/drop,
//! grouped-history, orientation-aid, and (when claimed) macro-replay proof is
//! present, reopenable, and inside its freshness window. A row that loses current
//! proof auto-narrows below its claim instead of coasting on an adjacent green
//! row.
//!
//! * a [`CertifiedSurfaceRow`] ties a durable [`KeyboardSurfaceSubject`] (keyed by
//!   a [`KeyboardSurfaceKind`], a [`SurfaceOriginClass`], and a non-display
//!   fingerprint, so a provider-linked surface never reads as a local one) to a
//!   list of [`DimensionCertification`] rows over the [`ParityDimension`]
//!   vocabulary, a claimed [`ContinuityParityGrade`], an effective grade, and —
//!   when narrowed — a [`ParityDowngradeTrigger`] plus a precise narrowed label;
//! * each [`DimensionCertification`] is **evidence-bound, not asserted**: it names
//!   an [`AxisProofCurrency`] and, unless the proof is missing, a reopenable
//!   `proof_ref` keyed by a non-display fingerprint, so certification review can
//!   reopen the same mode-strip / clipboard / drop / history / orientation
//!   evidence object that backs the grade;
//! * the row **auto-narrows**: [`CertifiedSurfaceRow::needs_narrow`] is true
//!   whenever a required-core dimension is uncertified or any certified dimension
//!   lacks current proof (stale, missing, requires-review, or imported proof
//!   standing in for a local claim). A narrowed row must carry an effective grade
//!   strictly below its claim, a recorded trigger, and a precise label — never a
//!   generic non-answer.
//!
//! [`InteractionParityCertificationPacket::validate`] also refuses a packet that
//! silently approximates an unsupported modal sequence, lets rich text become the
//! only copy representation, hides a drag/drop verb or its scope, flattens the
//! exact / compensating / checkpoint undo classes into one opaque history label,
//! drops orientation truth, or lets a provider-linked surface read as a locally
//! verified result.
//!
//! Raw provider payloads, file contents, raw clipboard / drag payload bodies,
//! credentials, and absolute private paths never cross this boundary; the packet
//! carries only typed class tokens, booleans, opaque / relative ids, fingerprint
//! digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.schema.json`](../../../../schemas/interaction/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.md`](../../../../docs/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/`](../../../../fixtures/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Re-export the frozen taxonomy this certification binds, so product, help,
// accessibility, support, and migration surfaces can name those types through
// this module rather than reaching into the matrix module by hand.
pub use crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::{
    AxisProofCurrency, AxisVerification, ContinuityParityGrade, KeyboardSurfaceKind,
    KeyboardSurfaceSubject, ParityDowngradeTrigger, SurfaceOriginClass,
    KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
};

/// Stable record-kind tag carried by [`InteractionParityCertificationPacket`].
pub const INTERACTION_PARITY_CERTIFICATION_RECORD_KIND: &str =
    "certify_keyboard_first_modal_parity_clipboard_drop_grouped_history_orientation_packet";

/// Schema version for the interaction-parity certification packet.
pub const INTERACTION_PARITY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const INTERACTION_PARITY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/interaction/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.schema.json";

/// Repo-relative path of the contract doc.
pub const INTERACTION_PARITY_CERTIFICATION_DOC_REF: &str =
    "docs/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.md";

/// Repo-relative path of the checked support-export artifact.
pub const INTERACTION_PARITY_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const INTERACTION_PARITY_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.md";

/// Repo-relative path of the protected fixture directory.
pub const INTERACTION_PARITY_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie";

/// One interaction-safety dimension a claimed switching / power-user row is
/// certified against. The first four are the **required core** every claimed row
/// must certify; the rest are quality dimensions a row certifies only when it
/// claims them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityDimension {
    /// Modal mode state, leader-key sequences, and keyboard completeness.
    ModalKeyboardParity,
    /// Clipboard route (plain-text-preserving copy) and drag/drop verb / scope
    /// disclosure.
    ClipboardDropSafety,
    /// Grouped history: distinct exact / compensating / checkpoint undo classes
    /// and reopen / recover continuity.
    GroupedHistoryContinuity,
    /// Orientation aids (multi-cursor, fold-state, breadcrumb, minimap,
    /// overview-ruler) that degrade honestly rather than silently disappearing.
    OrientationAidContinuity,
    /// Macro-replay review / downgrade for run-capable or cross-file replays.
    MacroReplaySafety,
}

impl ParityDimension {
    /// Every parity dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ModalKeyboardParity,
        Self::ClipboardDropSafety,
        Self::GroupedHistoryContinuity,
        Self::OrientationAidContinuity,
        Self::MacroReplaySafety,
    ];

    /// The required-core dimensions every claimed switching / power-user row must
    /// certify.
    pub const REQUIRED_CORE: [Self; 4] = [
        Self::ModalKeyboardParity,
        Self::ClipboardDropSafety,
        Self::GroupedHistoryContinuity,
        Self::OrientationAidContinuity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModalKeyboardParity => "modal_keyboard_parity",
            Self::ClipboardDropSafety => "clipboard_drop_safety",
            Self::GroupedHistoryContinuity => "grouped_history_continuity",
            Self::OrientationAidContinuity => "orientation_aid_continuity",
            Self::MacroReplaySafety => "macro_replay_safety",
        }
    }

    /// Whether this dimension is part of the required core.
    pub const fn is_core(self) -> bool {
        matches!(
            self,
            Self::ModalKeyboardParity
                | Self::ClipboardDropSafety
                | Self::GroupedHistoryContinuity
                | Self::OrientationAidContinuity
        )
    }
}

/// One dimension's certification: the proof currency plus a reopenable evidence
/// object, so a grade is backed by an object a reviewer can reopen rather than an
/// asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionCertification {
    /// Dimension being certified.
    pub dimension: ParityDimension,
    /// Currency of the proof backing this dimension, plus its reopenable ref.
    pub verification: AxisVerification,
}

impl DimensionCertification {
    /// Whether the proof object is reopenable.
    pub fn proof_reopenable(&self) -> bool {
        self.verification.proof_reopenable()
    }

    /// Whether this certification is well-formed.
    pub fn is_well_formed(&self) -> bool {
        self.verification.is_well_formed()
    }

    /// Whether this certification backs a current claim for the given surface
    /// imported posture.
    pub fn backs_claim(&self, provider_or_imported: bool) -> bool {
        self.verification.backs_claim(provider_or_imported)
    }
}

/// One claimed M5 switching / power-user surface certified against its
/// interaction-safety dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the row certifies.
    pub subject: KeyboardSurfaceSubject,
    /// Human-readable row label.
    pub label_summary: String,
    /// True when the surface is provider-linked / imported and must never read as
    /// a locally verified result.
    pub imported_surface: bool,
    /// Per-dimension certifications.
    pub certifications: Vec<DimensionCertification>,
    /// Whether unsupported modal sequences are downgraded honestly rather than
    /// silently approximated.
    pub modal_sequences_never_silently_approximated: bool,
    /// Whether a useful plain-text copy representation is preserved so rich text
    /// is never the only readable output.
    pub plain_text_copy_preserved: bool,
    /// Whether every drag/drop verb and its insertion / window scope is disclosed
    /// before commit rather than hidden behind a destructive default.
    pub drag_drop_verbs_and_scope_disclosed: bool,
    /// Whether the exact / compensating / checkpoint undo classes stay distinct
    /// rather than collapsing into one opaque history label.
    pub undo_classes_distinct: bool,
    /// Whether orientation aids degrade honestly with a precise reason label
    /// rather than silently disappearing.
    pub orientation_aids_degrade_honestly: bool,
    /// Headline parity grade publicly claimed for this row.
    pub claimed_grade: ContinuityParityGrade,
    /// Effective grade after auto-narrowing; equals the claim when every dimension
    /// is current, and ranks strictly below it otherwise.
    pub effective_grade: ContinuityParityGrade,
    /// Trigger that fired the narrow, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<ParityDowngradeTrigger>,
    /// Precise narrowed label, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl CertifiedSurfaceRow {
    /// Dimensions certified by this row.
    pub fn certified_dimensions(&self) -> BTreeSet<ParityDimension> {
        self.certifications.iter().map(|c| c.dimension).collect()
    }

    /// Resolves a certification by dimension.
    pub fn certification(&self, dimension: ParityDimension) -> Option<&DimensionCertification> {
        self.certifications
            .iter()
            .find(|c| c.dimension == dimension)
    }

    /// Whether every required-core dimension is certified.
    pub fn has_all_required_core(&self) -> bool {
        let certified = self.certified_dimensions();
        ParityDimension::REQUIRED_CORE
            .iter()
            .all(|dimension| certified.contains(dimension))
    }

    /// Whether the row carries a public parity claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_grade.is_claimed()
    }

    /// Whether every certified dimension backs a current claim for this row's
    /// imported posture.
    pub fn all_dimensions_current(&self) -> bool {
        self.certifications
            .iter()
            .all(|c| c.backs_claim(self.imported_surface))
    }

    /// Whether the row must narrow below its claim because a required-core
    /// dimension is uncertified or any certified dimension lacks current proof.
    pub fn needs_narrow(&self) -> bool {
        !self.has_all_required_core() || !self.all_dimensions_current()
    }

    /// Whether the effective grade and narrow evidence are consistent.
    ///
    /// When every dimension is current the effective grade equals the claim;
    /// otherwise it must rank strictly below the claim and carry both a recorded
    /// trigger and a precise narrowed label.
    pub fn narrow_consistent(&self) -> bool {
        if self.needs_narrow() {
            self.effective_grade.rank() < self.claimed_grade.rank()
                && self.narrow_trigger.is_some()
                && self
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_grade == self.claimed_grade
        }
    }

    /// Whether the imported posture is consistent: the row flag and its subject
    /// origin agree, so an imported / provider-linked surface never reads as a
    /// local result.
    pub fn imported_posture_consistent(&self) -> bool {
        self.imported_surface == self.subject.is_provider_or_imported()
    }

    /// Whether every per-row safety guardrail holds.
    pub fn safety_guardrails_hold(&self) -> bool {
        self.modal_sequences_never_silently_approximated
            && self.plain_text_copy_preserved
            && self.drag_drop_verbs_and_scope_disclosed
            && self.undo_classes_distinct
            && self.orientation_aids_degrade_honestly
    }

    /// Whether every field required to record this row is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.subject.is_valid()
            && !self.certifications.is_empty()
            && self
                .certifications
                .iter()
                .all(DimensionCertification::is_well_formed)
            && self.narrow_consistent()
            && self.imported_posture_consistent()
            && self.safety_guardrails_hold()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionParityGuardrails {
    /// Unsupported modal sequences are never silently approximated.
    pub modal_sequences_never_silently_approximated: bool,
    /// Rich text is never the only copy representation; plain text is preserved.
    pub plain_text_copy_never_lost: bool,
    /// Drag/drop never becomes destructive or ambiguous by default; verbs and
    /// scope are disclosed.
    pub drag_drop_never_destructive_or_ambiguous: bool,
    /// Exact undo, compensating action, and checkpoint restore are never flattened
    /// into one vague history label.
    pub undo_classes_never_flattened: bool,
    /// Orientation aids degrade honestly and are never silently removed.
    pub orientation_aids_degrade_honestly: bool,
    /// Any claimed row lacking current proof auto-narrows below its claim.
    pub rows_auto_narrow_without_current_proof: bool,
    /// No new general macro language or editor core is introduced here.
    pub no_new_macro_language_or_editor_core: bool,
}

impl InteractionParityGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.modal_sequences_never_silently_approximated
            && self.plain_text_copy_never_lost
            && self.drag_drop_never_destructive_or_ambiguous
            && self.undo_classes_never_flattened
            && self.orientation_aids_degrade_honestly
            && self.rows_auto_narrow_without_current_proof
            && self.no_new_macro_language_or_editor_core
    }
}

/// Consumer projection block: the surfaces that read this certification without
/// re-deriving switching-wedge maturity by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionParityConsumerProjection {
    /// Product switching / power-user surfaces ingest this certification.
    pub product_ingests_certification: bool,
    /// Help / migration ingests the same certification.
    pub help_migration_ingests_certification: bool,
    /// Accessibility surfaces ingest the same certification.
    pub accessibility_ingests_certification: bool,
    /// Support / export ingests the same certification.
    pub support_ingests_certification: bool,
    /// Release-control surfaces ingest the same certification.
    pub release_control_ingests_certification: bool,
    /// Narrowed rows are visibly labeled below their claim in every surface.
    pub narrowed_rows_labeled_below_claim: bool,
}

impl InteractionParityConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_certification
            && self.help_migration_ingests_certification
            && self.accessibility_ingests_certification
            && self.support_ingests_certification
            && self.release_control_ingests_certification
            && self.narrowed_rows_labeled_below_claim
    }
}

/// Evidence freshness block for the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionParityFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically narrows claimed rows.
    pub auto_narrow_on_stale: bool,
}

impl InteractionParityFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.evidence_freshness_slo_hours > 0 && !self.last_evidence_refresh.trim().is_empty()
    }
}

/// Constructor input for [`InteractionParityCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionParityCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: InteractionParityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: InteractionParityConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: InteractionParityFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe interaction-parity certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionParityCertificationPacket {
    /// Record kind; must equal [`INTERACTION_PARITY_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`INTERACTION_PARITY_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: InteractionParityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: InteractionParityConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: InteractionParityFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl InteractionParityCertificationPacket {
    /// Builds an interaction-parity certification packet.
    pub fn new(input: InteractionParityCertificationPacketInput) -> Self {
        Self {
            record_kind: INTERACTION_PARITY_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: INTERACTION_PARITY_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some row in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<KeyboardSurfaceKind> {
        self.rows.iter().map(|row| row.surface_kind).collect()
    }

    /// Parity dimensions certified by some row in this packet.
    pub fn represented_dimensions(&self) -> BTreeSet<ParityDimension> {
        self.rows
            .iter()
            .flat_map(|row| row.certified_dimensions())
            .collect()
    }

    /// Proof currencies represented across certifications.
    pub fn represented_currencies(&self) -> BTreeSet<AxisProofCurrency> {
        self.rows
            .iter()
            .flat_map(|row| {
                row.certifications
                    .iter()
                    .map(|c| c.verification.proof_currency)
            })
            .collect()
    }

    /// Surface origin classes represented across rows.
    pub fn represented_origin_classes(&self) -> BTreeSet<SurfaceOriginClass> {
        self.rows
            .iter()
            .map(|row| row.subject.origin_class)
            .collect()
    }

    /// Count of rows that auto-narrowed below their claim.
    pub fn narrowed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_narrow()).count()
    }

    /// Count of rows holding a public parity claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Count of imported / provider-linked rows.
    pub fn imported_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.imported_surface).count()
    }

    /// Resolves a row by its id.
    pub fn row(&self, row_id: &str) -> Option<&CertifiedSurfaceRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Validates the interaction-parity certification invariants.
    pub fn validate(&self) -> Vec<InteractionParityCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != INTERACTION_PARITY_CERTIFICATION_RECORD_KIND {
            violations.push(InteractionParityCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != INTERACTION_PARITY_CERTIFICATION_SCHEMA_VERSION {
            violations.push(InteractionParityCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(InteractionParityCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(InteractionParityCertificationViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(InteractionParityCertificationViolation::ConsumerProjectionIncomplete);
        }
        if !self.evidence_freshness.is_valid() {
            violations.push(InteractionParityCertificationViolation::EvidenceFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("interaction parity certification packet serializes"),
        ) {
            violations.push(InteractionParityCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("interaction parity certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, accessibility, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Keyboard-First Modal Parity / Clipboard-Drop / Grouped-History / Orientation-Aid Certification\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} imported, {} narrowed)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.imported_row_count(),
            self.narrowed_row_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Dimensions certified: {} / {}\n",
            self.represented_dimensions().len(),
            ParityDimension::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.surface_kind.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - subject `{}` ({}), imported={}\n",
                row.subject.surface_id,
                row.subject.origin_class.as_str(),
                row.imported_surface
            ));
            for cert in &row.certifications {
                out.push_str(&format!(
                    "  - {} = `{}`\n",
                    cert.dimension.as_str(),
                    cert.verification.proof_currency.as_str()
                ));
            }
            if let Some(label) = &row.narrowed_label {
                out.push_str(&format!("  - Narrowed: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum InteractionParityCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<InteractionParityCertificationViolation>),
}

impl fmt::Display for InteractionParityCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "interaction parity certification export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "interaction parity certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for InteractionParityCertificationArtifactError {}

/// Validation failures emitted by [`InteractionParityCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionParityCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no row.
    RequiredSurfaceKindMissing,
    /// Some required-core dimension is certified by no row.
    DimensionCoverageMissing,
    /// No row demonstrates auto-narrowing on uncurrent proof.
    NarrowedRowCaseMissing,
    /// No row certifies current proof.
    CurrentProofCaseMissing,
    /// No imported / provider-linked row is present.
    ImportedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not narrowed below its claim despite uncurrent proof.
    RowNotNarrowedOnUncurrentProof,
    /// A narrowed row lacks a precise narrowed label or trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// A row's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// An unsupported modal sequence was silently approximated.
    ModalSequenceApproximated,
    /// Rich text was offered as the only copy representation.
    PlainTextCopyLost,
    /// A drag/drop verb or its scope was hidden.
    DragDropVerbHidden,
    /// The exact / compensating / checkpoint undo classes were flattened.
    UndoClassesFlattened,
    /// Orientation truth was dropped rather than degraded honestly.
    OrientationTruthDropped,
    /// An imported / provider-linked row reads as a live local result.
    ImportedReadsAsLocal,
    /// A dimension proof is not reopenable (missing ref or fingerprint substitutes).
    DimensionProofNotReopenable,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl InteractionParityCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::DimensionCoverageMissing => "dimension_coverage_missing",
            Self::NarrowedRowCaseMissing => "narrowed_row_case_missing",
            Self::CurrentProofCaseMissing => "current_proof_case_missing",
            Self::ImportedRowCaseMissing => "imported_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotNarrowedOnUncurrentProof => "row_not_narrowed_on_uncurrent_proof",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::ModalSequenceApproximated => "modal_sequence_approximated",
            Self::PlainTextCopyLost => "plain_text_copy_lost",
            Self::DragDropVerbHidden => "drag_drop_verb_hidden",
            Self::UndoClassesFlattened => "undo_classes_flattened",
            Self::OrientationTruthDropped => "orientation_truth_dropped",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::DimensionProofNotReopenable => "dimension_proof_not_reopenable",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_interaction_parity_certification_export(
) -> Result<InteractionParityCertificationPacket, InteractionParityCertificationArtifactError> {
    let packet: InteractionParityCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/support_export.json"
    )))
    .map_err(InteractionParityCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(InteractionParityCertificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &InteractionParityCertificationPacket,
    violations: &mut Vec<InteractionParityCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        INTERACTION_PARITY_CERTIFICATION_SCHEMA_REF,
        INTERACTION_PARITY_CERTIFICATION_DOC_REF,
        INTERACTION_PARITY_CERTIFICATION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(InteractionParityCertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &InteractionParityCertificationPacket,
    violations: &mut Vec<InteractionParityCertificationViolation>,
) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in KeyboardSurfaceKind::ALL {
        if !surface_kinds.contains(&required) {
            violations.push(InteractionParityCertificationViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let dimensions = packet.represented_dimensions();
    for required in ParityDimension::REQUIRED_CORE {
        if !dimensions.contains(&required) {
            violations.push(InteractionParityCertificationViolation::DimensionCoverageMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_narrow() && row.narrow_consistent())
    {
        violations.push(InteractionParityCertificationViolation::NarrowedRowCaseMissing);
    }

    let currencies = packet.represented_currencies();
    if !currencies
        .iter()
        .any(|currency| currency.is_current_local() || currency.is_imported_current())
    {
        violations.push(InteractionParityCertificationViolation::CurrentProofCaseMissing);
    }

    if packet.imported_row_count() == 0 {
        violations.push(InteractionParityCertificationViolation::ImportedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &InteractionParityCertificationPacket,
    violations: &mut Vec<InteractionParityCertificationViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(InteractionParityCertificationViolation::RowIncomplete);
        }
        if row.needs_narrow() && row.effective_grade.rank() >= row.claimed_grade.rank() {
            violations
                .push(InteractionParityCertificationViolation::RowNotNarrowedOnUncurrentProof);
        }
        if row.needs_narrow()
            && (row.narrow_trigger.is_none()
                || !row
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(InteractionParityCertificationViolation::NarrowedRowMissingLabelOrTrigger);
        }
        if !row.subject.fingerprint_independent_of_id() {
            violations
                .push(InteractionParityCertificationViolation::FingerprintSubstitutesIdentity);
        }
        if !row.modal_sequences_never_silently_approximated {
            violations.push(InteractionParityCertificationViolation::ModalSequenceApproximated);
        }
        if !row.plain_text_copy_preserved {
            violations.push(InteractionParityCertificationViolation::PlainTextCopyLost);
        }
        if !row.drag_drop_verbs_and_scope_disclosed {
            violations.push(InteractionParityCertificationViolation::DragDropVerbHidden);
        }
        if !row.undo_classes_distinct {
            violations.push(InteractionParityCertificationViolation::UndoClassesFlattened);
        }
        if !row.orientation_aids_degrade_honestly {
            violations.push(InteractionParityCertificationViolation::OrientationTruthDropped);
        }
        if !row.imported_posture_consistent() {
            violations.push(InteractionParityCertificationViolation::ImportedReadsAsLocal);
        }
        if row.certifications.iter().any(|cert| !cert.is_well_formed()) {
            violations.push(InteractionParityCertificationViolation::DimensionProofNotReopenable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(InteractionParityCertificationViolation::RowEvidenceMissing);
        }
    }
}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise narrow truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
            | "downgraded"
            | "unverified"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Packet id used by [`seeded_interaction_parity_certification_packet`].
pub const SEED_INTERACTION_PARITY_CERTIFICATION_PACKET_ID: &str =
    "m5-interaction-parity-certification:stable:0001";

/// Mint timestamp used by [`seeded_interaction_parity_certification_packet`].
pub const SEED_INTERACTION_PARITY_CERTIFICATION_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating interaction-parity certification packet that
/// the checked-in support export, the Markdown summary, and the conformance tests
/// all share, so the in-crate builder stays byte-aligned with the artifact.
///
/// The seed certifies one row per claimed M5 surface kind: a fully release-bearing
/// editor core, fully parity-complete notebook and data/API surfaces, a preview
/// surface and a docs surface, a review surface, a runtime surface, and a
/// provider-linked companion surface whose imported proof backs only its imported
/// claim — plus a docs-surface row that auto-narrows because its orientation-aid
/// proof has aged outside its freshness window.
pub fn seeded_interaction_parity_certification_packet() -> InteractionParityCertificationPacket {
    InteractionParityCertificationPacket::new(InteractionParityCertificationPacketInput {
        packet_id: SEED_INTERACTION_PARITY_CERTIFICATION_PACKET_ID.to_owned(),
        label: "M5 Keyboard-First Modal Parity, Clipboard-Drop Safety, Grouped-History Honesty, and Orientation-Aid Continuity Certification"
            .to_owned(),
        rows: seeded_rows(),
        guardrails: InteractionParityGuardrails {
            modal_sequences_never_silently_approximated: true,
            plain_text_copy_never_lost: true,
            drag_drop_never_destructive_or_ambiguous: true,
            undo_classes_never_flattened: true,
            orientation_aids_degrade_honestly: true,
            rows_auto_narrow_without_current_proof: true,
            no_new_macro_language_or_editor_core: true,
        },
        consumer_projection: InteractionParityConsumerProjection {
            product_ingests_certification: true,
            help_migration_ingests_certification: true,
            accessibility_ingests_certification: true,
            support_ingests_certification: true,
            release_control_ingests_certification: true,
            narrowed_rows_labeled_below_claim: true,
        },
        evidence_freshness: InteractionParityFreshness {
            evidence_freshness_slo_hours: 168,
            last_evidence_refresh: SEED_INTERACTION_PARITY_CERTIFICATION_MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            INTERACTION_PARITY_CERTIFICATION_SCHEMA_REF.to_owned(),
            INTERACTION_PARITY_CERTIFICATION_DOC_REF.to_owned(),
            INTERACTION_PARITY_CERTIFICATION_ARTIFACT_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_INTERACTION_PARITY_CERTIFICATION_MINTED_AT.to_owned(),
    })
}

/// Builds the protected narrow-drill fixture: a claimed editor-core row whose
/// grouped-history proof has aged outside its freshness window, so the row
/// auto-narrows below its switching-certified claim.
pub fn fixture_interaction_parity_certification_packet() -> InteractionParityCertificationPacket {
    let mut packet = seeded_interaction_parity_certification_packet();
    packet.packet_id = "m5-interaction-parity-certification:fixture:stale-history".to_owned();
    packet.label =
        "M5 Interaction Parity Certification: editor-core row narrows on stale grouped-history proof"
            .to_owned();

    let editor = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::EditorCore)
        .expect("seed carries an editor-core row");
    for cert in &mut editor.certifications {
        if cert.dimension == ParityDimension::GroupedHistoryContinuity {
            cert.verification.proof_currency = AxisProofCurrency::StaleExpired;
        }
    }
    editor.claimed_grade = ContinuityParityGrade::SwitchingCertified;
    editor.effective_grade = ContinuityParityGrade::ParityUnverified;
    editor.narrow_trigger = Some(ParityDowngradeTrigger::StaleVerificationProof);
    editor.narrowed_label = Some(
        "Grouped-history proof aged past its freshness window; editor-core parity held at unverified until reverified"
            .to_owned(),
    );
    packet
}

fn seeded_rows() -> Vec<CertifiedSurfaceRow> {
    vec![
        editor_core_release_row(),
        notebook_parity_complete_row(),
        data_api_parity_complete_row(),
        preview_parity_complete_row(),
        docs_narrowed_row(),
        review_parity_complete_row(),
        runtime_parity_complete_row(),
        companion_imported_row(),
    ]
}

/// Builds a verification proof keyed by a non-display fingerprint distinct from
/// the row id and dimension.
fn proof_for(
    row_id: &str,
    dimension: ParityDimension,
    currency: AxisProofCurrency,
    summary: &str,
) -> AxisVerification {
    let (proof_ref, proof_fingerprint_token) = if currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{row_id}:{}", dimension.as_str())),
            Some(format!("fp:proof:{row_id}:{}", dimension.as_str())),
        )
    };
    AxisVerification {
        proof_currency: currency,
        proof_ref,
        proof_fingerprint_token,
        summary: summary.to_owned(),
    }
}

/// Builds a subject whose fingerprint is independent of its surface id.
fn subject_for(row_id: &str, origin_class: SurfaceOriginClass) -> KeyboardSurfaceSubject {
    KeyboardSurfaceSubject {
        surface_id: format!("surface:{row_id}"),
        origin_class,
        surface_fingerprint_token: format!("fp:surface:{row_id}"),
    }
}

/// Builds a row whose every claimed dimension is backed by current local proof.
fn local_parity_row(
    row_id: &str,
    surface_kind: KeyboardSurfaceKind,
    label_summary: &str,
    claimed_grade: ContinuityParityGrade,
    dimensions: &[ParityDimension],
) -> CertifiedSurfaceRow {
    let certifications = dimensions
        .iter()
        .map(|dimension| DimensionCertification {
            dimension: *dimension,
            verification: proof_for(
                row_id,
                *dimension,
                AxisProofCurrency::VerifiedCurrent,
                &format!(
                    "{} proof reverified inside its freshness window",
                    dimension.as_str()
                ),
            ),
        })
        .collect();
    CertifiedSurfaceRow {
        row_id: row_id.to_owned(),
        surface_kind,
        subject: subject_for(row_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: label_summary.to_owned(),
        imported_surface: false,
        certifications,
        modal_sequences_never_silently_approximated: true,
        plain_text_copy_preserved: true,
        drag_drop_verbs_and_scope_disclosed: true,
        undo_classes_distinct: true,
        orientation_aids_degrade_honestly: true,
        claimed_grade,
        effective_grade: claimed_grade,
        narrow_trigger: None,
        narrowed_label: None,
        evidence_refs: vec![format!("evidence:{row_id}")],
        source_contract_refs: vec![
            INTERACTION_PARITY_CERTIFICATION_DOC_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
        ],
    }
}

fn editor_core_release_row() -> CertifiedSurfaceRow {
    local_parity_row(
        "interaction-cert:editor-core:0001",
        KeyboardSurfaceKind::EditorCore,
        "Editor core: full modal parity, plain-text copy, distinct undo classes, live orientation aids, reviewed macro replay",
        ContinuityParityGrade::SwitchingCertified,
        &ParityDimension::ALL,
    )
}

fn notebook_parity_complete_row() -> CertifiedSurfaceRow {
    local_parity_row(
        "interaction-cert:notebook:0001",
        KeyboardSurfaceKind::NotebookSurface,
        "Notebook surface: cell-scoped modal navigation, plain-text cell copy, grouped cell history, fold-state orientation aids",
        ContinuityParityGrade::ParityComplete,
        &ParityDimension::REQUIRED_CORE,
    )
}

fn data_api_parity_complete_row() -> CertifiedSurfaceRow {
    local_parity_row(
        "interaction-cert:data-api:0001",
        KeyboardSurfaceKind::DataApiSurface,
        "Data/API surface: keyboard-complete result grid, plain-text row copy with drop-verb disclosure, grouped query history",
        ContinuityParityGrade::ParityComplete,
        &ParityDimension::REQUIRED_CORE,
    )
}

fn preview_parity_complete_row() -> CertifiedSurfaceRow {
    local_parity_row(
        "interaction-cert:preview:0001",
        KeyboardSurfaceKind::PreviewSurface,
        "Preview surface: source-first keyboard navigation, plain-text copy, breadcrumb orientation aligned to source identity",
        ContinuityParityGrade::ParityComplete,
        &ParityDimension::REQUIRED_CORE,
    )
}

fn docs_narrowed_row() -> CertifiedSurfaceRow {
    let row_id = "interaction-cert:docs:0001";
    let mut certifications = vec![
        DimensionCertification {
            dimension: ParityDimension::ModalKeyboardParity,
            verification: proof_for(
                row_id,
                ParityDimension::ModalKeyboardParity,
                AxisProofCurrency::VerifiedCurrent,
                "docs authoring modal navigation reverified inside its freshness window",
            ),
        },
        DimensionCertification {
            dimension: ParityDimension::ClipboardDropSafety,
            verification: proof_for(
                row_id,
                ParityDimension::ClipboardDropSafety,
                AxisProofCurrency::VerifiedCurrent,
                "docs copy preserves plain text and discloses drop verbs",
            ),
        },
        DimensionCertification {
            dimension: ParityDimension::GroupedHistoryContinuity,
            verification: proof_for(
                row_id,
                ParityDimension::GroupedHistoryContinuity,
                AxisProofCurrency::VerifiedCurrent,
                "docs grouped history keeps exact / compensating / checkpoint classes distinct",
            ),
        },
        DimensionCertification {
            dimension: ParityDimension::OrientationAidContinuity,
            verification: proof_for(
                row_id,
                ParityDimension::OrientationAidContinuity,
                AxisProofCurrency::StaleExpired,
                "docs outline orientation-aid proof aged outside its freshness window",
            ),
        },
    ];
    certifications.sort_by_key(|c| c.dimension);
    CertifiedSurfaceRow {
        row_id: row_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(row_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Docs surface: parity claimed but orientation-aid proof is stale, so the row auto-narrows below its claim"
                .to_owned(),
        imported_surface: false,
        certifications,
        modal_sequences_never_silently_approximated: true,
        plain_text_copy_preserved: true,
        drag_drop_verbs_and_scope_disclosed: true,
        undo_classes_distinct: true,
        orientation_aids_degrade_honestly: true,
        claimed_grade: ContinuityParityGrade::ParityComplete,
        effective_grade: ContinuityParityGrade::ParityUnverified,
        narrow_trigger: Some(ParityDowngradeTrigger::StaleVerificationProof),
        narrowed_label: Some(
            "Orientation-aid proof aged past its freshness window; docs parity held at unverified until the breadcrumb / outline aids are reverified"
                .to_owned(),
        ),
        evidence_refs: vec![format!("evidence:{row_id}")],
        source_contract_refs: vec![
            INTERACTION_PARITY_CERTIFICATION_DOC_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
        ],
    }
}

fn review_parity_complete_row() -> CertifiedSurfaceRow {
    local_parity_row(
        "interaction-cert:review:0001",
        KeyboardSurfaceKind::ReviewSurface,
        "Review surface: keyboard-complete diff panel, plain-text copy, grouped review history, overview-ruler orientation aids",
        ContinuityParityGrade::ParityComplete,
        &ParityDimension::REQUIRED_CORE,
    )
}

fn runtime_parity_complete_row() -> CertifiedSurfaceRow {
    local_parity_row(
        "interaction-cert:runtime:0001",
        KeyboardSurfaceKind::RuntimeSurface,
        "Runtime surface: keyboard-complete embedded runtime, plain-text copy with drop-verb disclosure, grouped runtime history",
        ContinuityParityGrade::ParityPartial,
        &ParityDimension::REQUIRED_CORE,
    )
}

fn companion_imported_row() -> CertifiedSurfaceRow {
    let row_id = "interaction-cert:companion:0001";
    let certifications = ParityDimension::REQUIRED_CORE
        .iter()
        .map(|dimension| DimensionCertification {
            dimension: *dimension,
            verification: proof_for(
                row_id,
                *dimension,
                AxisProofCurrency::ImportedCurrent,
                &format!(
                    "{} proof is provider-backed and read-only; it backs only this imported claim",
                    dimension.as_str()
                ),
            ),
        })
        .collect();
    CertifiedSurfaceRow {
        row_id: row_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::CompanionSurface,
        subject: subject_for(row_id, SurfaceOriginClass::ProviderLinkedSurface),
        label_summary:
            "Companion surface: provider-linked parity proof is read-only and never reads as a locally verified result"
                .to_owned(),
        imported_surface: true,
        certifications,
        modal_sequences_never_silently_approximated: true,
        plain_text_copy_preserved: true,
        drag_drop_verbs_and_scope_disclosed: true,
        undo_classes_distinct: true,
        orientation_aids_degrade_honestly: true,
        claimed_grade: ContinuityParityGrade::ParityPartial,
        effective_grade: ContinuityParityGrade::ParityPartial,
        narrow_trigger: None,
        narrowed_label: None,
        evidence_refs: vec![format!("evidence:{row_id}")],
        source_contract_refs: vec![
            INTERACTION_PARITY_CERTIFICATION_DOC_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
        ],
    }
}
