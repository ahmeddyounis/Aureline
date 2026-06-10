//! Certify notebook diff, review, export, collaboration, and experiment packets
//! and narrow unqualified rows.
//!
//! This module materializes the typed certification layer that binds the M5
//! notebook depth lanes — diff/review, export, collaboration, and experiment
//! lineage — to canonical qualification states, downgrade rules, rollback paths,
//! and automatic narrowing actions. It produces [`NotebookCertificationRow`]
//! records and the [`NotebookCertificationPacket`] checked-in artifact that
//! downstream docs, help, CI, and support surfaces ingest instead of cloning
//! status text.
//!
//! The central rule is that a claimed certification — `certified_current` — may
//! never be implied from prose alone. Every row must point to a current sub-packet
//! whose freshness, rollback path, and evidence still support it. When the
//! sub-packet goes stale, the rollback path is missing, or a gap is detected, the
//! row is **automatically narrowed** rather than left asserting a claim the
//! evidence no longer backs.
//!
//! The records and closed vocabularies here mirror the boundary schema at
//! `/schemas/notebook/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.schema.json`.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT appear
//! on any record carried here. Only opaque handles and closed-vocabulary tokens
//! cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every certification record carried by this module.
/// Bumped only on breaking payload changes; additive-optional fields do not bump
/// this value.
pub const NOTEBOOK_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookCertificationRow`] payloads.
pub const NOTEBOOK_CERTIFICATION_ROW_RECORD_KIND: &str = "notebook_certification_row";

/// Stable record-kind tag for the checked-in [`NotebookCertificationPacket`].
pub const NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND: &str = "notebook_certification_packet";

/// Repo-relative path to the checked-in certification packet JSON.
pub const NOTEBOOK_CERTIFICATION_PACKET_PATH: &str =
    "artifacts/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.json";

/// Embedded checked-in certification packet JSON.
pub const NOTEBOOK_CERTIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Certification lane kind. Names which M5 notebook depth lane a row
    /// certifies.
    NotebookCertificationLaneKind {
        DiffReview => "diff_review",
        Export => "export",
        Collaboration => "collaboration",
        Experiment => "experiment",
        Narrowing => "narrowing",
    }
);

closed_vocab!(
    /// Certification state class. Names the qualification state earned by a
    /// lane after the certification packet is applied.
    NotebookCertificationState {
        CertifiedCurrent => "certified_current",
        Incomplete => "incomplete",
        Stale => "stale",
        OnWaiver => "on_waiver",
        Blocked => "blocked",
        RuleMissing => "rule_missing",
        Narrowed => "narrowed",
    }
);

closed_vocab!(
    /// Downgrade reason class. Names why a lane was narrowed below its claimed
    /// label.
    NotebookCertificationDowngradeReason {
        FreshnessExpired => "freshness_expired",
        PacketMissing => "packet_missing",
        EvidenceStale => "evidence_stale",
        RollbackPathMissing => "rollback_path_missing",
        UnderqualifiedSubLane => "underqualified_sub_lane",
        PolicyBlocked => "policy_blocked",
    }
);

closed_vocab!(
    /// Rollback path state class. Names whether the lane's rollback path is
    /// defined, tested, exercised, or missing.
    NotebookCertificationRollbackPathState {
        Defined => "defined",
        Tested => "tested",
        Exercised => "exercised",
        Missing => "missing",
    }
);

closed_vocab!(
    /// Narrowing action class. Names the automation that applies when a lane
    /// narrows.
    NotebookCertificationNarrowingAction {
        AutomaticNarrowing => "automatic_narrowing",
        ManualHold => "manual_hold",
        EmergencyRollback => "emergency_rollback",
    }
);

/// Generic finding shape used by every certification validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationFinding {
    /// Stable check id (e.g. `notebook_certification_row.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, row id, packet id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl CertificationFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookCertificationRow`].
pub type NotebookCertificationRowFinding = CertificationFinding;

/// Typed validation finding for a [`NotebookCertificationPacket`].
pub type NotebookCertificationPacketFinding = CertificationFinding;

/// Per-lane certification row. Carries the lane kind, sub-packet ref,
/// certification state, rollback path state, downgrade reasons, narrowing
/// action, and freshness so that unqualified rows are explicitly narrowed
/// rather than silently left green.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCertificationRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_certification_schema_version: u32,
    /// Stable opaque row id.
    pub row_id: String,
    /// Certification lane kind.
    pub lane_kind: NotebookCertificationLaneKind,
    /// Opaque ref to the sub-packet this row certifies (e.g.
    /// `notebook_diff_packet_v1`).
    pub sub_packet_ref: String,
    /// Certification state class.
    pub certification_state: NotebookCertificationState,
    /// Rollback path state class.
    pub rollback_path_state: NotebookCertificationRollbackPathState,
    /// Downgrade reasons that narrowed this row, if any.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub downgrade_reasons: Vec<NotebookCertificationDowngradeReason>,
    /// Narrowing action applied when this row narrows.
    pub narrowing_action: NotebookCertificationNarrowingAction,
    /// UTC date this row's evidence is current as of.
    pub freshness_as_of: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCertificationRow {
    /// Returns typed truth-rule findings; an empty vector means the row is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookCertificationRowFinding> {
        let mut findings = Vec::new();
        let subject = self.row_id.as_str();

        if self.record_kind != NOTEBOOK_CERTIFICATION_ROW_RECORD_KIND {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_CERTIFICATION_ROW_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_certification_schema_version != NOTEBOOK_CERTIFICATION_SCHEMA_VERSION {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_CERTIFICATION_SCHEMA_VERSION}, found {}",
                    self.notebook_certification_schema_version
                ),
            ));
        }

        if self.row_id.trim().is_empty() {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.row_id_required",
                subject,
                "row_id must be non-empty",
            ));
        }

        if self.sub_packet_ref.trim().is_empty() {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.sub_packet_ref_required",
                subject,
                "sub_packet_ref must be non-empty",
            ));
        }

        if self.freshness_as_of.trim().is_empty() {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.freshness_as_of_required",
                subject,
                "freshness_as_of must be non-empty",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        // A certified_current row must have a tested or exercised rollback path
        // and no downgrade reasons.
        if self.certification_state == NotebookCertificationState::CertifiedCurrent {
            if !self.rollback_path_state.holds_label() {
                findings.push(NotebookCertificationRowFinding::new(
                    "notebook_certification_row.rollback_path_insufficient",
                    subject,
                    "certified_current requires rollback_path_state to be tested or exercised",
                ));
            }
            if !self.downgrade_reasons.is_empty() {
                findings.push(NotebookCertificationRowFinding::new(
                    "notebook_certification_row.no_downgrade_for_certified",
                    subject,
                    "certified_current row must not carry downgrade reasons",
                ));
            }
        }

        // A narrowed row must carry at least one downgrade reason.
        if self.certification_state == NotebookCertificationState::Narrowed
            && self.downgrade_reasons.is_empty()
        {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.narrowed_requires_reason",
                subject,
                "narrowed row must carry at least one downgrade reason",
            ));
        }

        // A missing rollback path forces narrowing below certified_current.
        if self.rollback_path_state == NotebookCertificationRollbackPathState::Missing
            && self.certification_state == NotebookCertificationState::CertifiedCurrent
        {
            findings.push(NotebookCertificationRowFinding::new(
                "notebook_certification_row.missing_rollback_forbids_certified",
                subject,
                "missing rollback_path forbids certified_current",
            ));
        }

        findings
    }
}

/// Checked-in certification packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCertificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Certification rows for each M5 notebook depth lane.
    pub certification_rows: Vec<NotebookCertificationRow>,
    /// Worked example narrowed rows showing automatic narrowing.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub example_narrowed_rows: Vec<NotebookCertificationRow>,
    /// Human-readable downgrade rules.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub downgrade_rules: Vec<String>,
    /// Human-readable rollback path steps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rollback_path: Vec<String>,
    /// Maximum age in days before the packet is considered stale.
    pub freshness_slo_max_age_days: u32,
    /// Days before expiry when a warning is issued.
    pub warn_window_days: u32,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCertificationPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookCertificationPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_CERTIFICATION_SCHEMA_VERSION {
            findings.push(NotebookCertificationPacketFinding::new(
                "notebook_certification_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_CERTIFICATION_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND {
            findings.push(NotebookCertificationPacketFinding::new(
                "notebook_certification_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND, self.record_kind
                ),
            ));
        }

        if self.packet_id.trim().is_empty() {
            findings.push(NotebookCertificationPacketFinding::new(
                "notebook_certification_packet.packet_id_required",
                subject,
                "packet_id must be non-empty",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookCertificationPacketFinding::new(
                "notebook_certification_packet.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        // Every lane kind must be represented exactly once in certification_rows.
        let mut seen_lanes = std::collections::HashSet::new();
        for row in &self.certification_rows {
            if !seen_lanes.insert(row.lane_kind) {
                findings.push(NotebookCertificationPacketFinding::new(
                    "notebook_certification_packet.duplicate_lane",
                    subject,
                    format!(
                        "duplicate certification row for lane {}",
                        row.lane_kind.as_str()
                    ),
                ));
            }
            findings.extend(row.validate().into_iter().map(|f| {
                NotebookCertificationPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for expected in NotebookCertificationLaneKind::ALL {
            if !seen_lanes.contains(&expected) {
                findings.push(NotebookCertificationPacketFinding::new(
                    "notebook_certification_packet.missing_lane",
                    subject,
                    format!("missing certification row for lane {}", expected.as_str()),
                ));
            }
        }

        for row in &self.example_narrowed_rows {
            findings.extend(row.validate().into_iter().map(|f| {
                NotebookCertificationPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Parses the checked-in certification packet JSON.
pub fn current_notebook_certification_packet(
) -> Result<NotebookCertificationPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_CERTIFICATION_PACKET_JSON)
}

impl NotebookCertificationLaneKind {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DiffReview,
        Self::Export,
        Self::Collaboration,
        Self::Experiment,
        Self::Narrowing,
    ];
}

impl NotebookCertificationState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CertifiedCurrent,
        Self::Incomplete,
        Self::Stale,
        Self::OnWaiver,
        Self::Blocked,
        Self::RuleMissing,
        Self::Narrowed,
    ];
}

impl NotebookCertificationDowngradeReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FreshnessExpired,
        Self::PacketMissing,
        Self::EvidenceStale,
        Self::RollbackPathMissing,
        Self::UnderqualifiedSubLane,
        Self::PolicyBlocked,
    ];
}

impl NotebookCertificationRollbackPathState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Defined, Self::Tested, Self::Exercised, Self::Missing];

    /// Whether the state allows the lane to hold a `certified_current` label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Tested | Self::Exercised)
    }

    /// Whether the state forces narrowing below `certified_current`.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

impl NotebookCertificationNarrowingAction {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::AutomaticNarrowing,
        Self::ManualHold,
        Self::EmergencyRollback,
    ];
}

#[cfg(test)]
mod tests;
