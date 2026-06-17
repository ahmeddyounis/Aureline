//! Desktop search export and support-replay chrome bound to the search-export
//! governance packet.
//!
//! This module is the desktop first consumer of
//! [`SearchExportGovernancePacket`](aureline_search::SearchExportGovernancePacket).
//! It projects compact, inspectable replay cues for the support/export and
//! replay surfaces, reusing the canonical export packets verbatim so the chrome
//! renders one truth with the CLI/headless, support-export, and managed-analytics
//! consumers.
//!
//! The projection is privacy-safe by construction: a cue never carries literal
//! query text. It surfaces the export class, redaction mode, loaded/hidden/omitted
//! counts and omission flags, the query-session and evidence refs, and the
//! replay-safety disclosure, so the chrome can explain what search ran, what was
//! selected, and what was omitted without storing the literal query string and
//! without ever claiming live current results.

use aureline_search::{
    ExportConsentClass, SearchExportClass, SearchExportConsumerClass, SearchExportGovernancePacket,
    SearchExportSnapshotTruth, SearchPacketRedactionState, SearchResultSemantics,
    SearchScopeHonestyState,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SearchExportGovernanceProjectionSet`].
pub const SEARCH_EXPORT_GOVERNANCE_PROJECTION_SET_RECORD_KIND: &str =
    "search_export_governance_projection_set";

/// Schema version for [`SearchExportGovernanceProjectionSet`].
pub const SEARCH_EXPORT_GOVERNANCE_PROJECTION_SET_SCHEMA_VERSION: u32 = 1;

/// One desktop replay cue bound to a single governed export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportReplayCue {
    /// Row id reused from the substrate.
    pub row_id: String,
    /// Reviewable scenario summary (never literal query text).
    pub scenario: String,
    /// Export class shown on the replay cue.
    pub export_class: SearchExportClass,
    /// Redaction mode shown on the replay cue.
    pub redaction_state: SearchPacketRedactionState,
    /// Literal-query consent posture shown on the replay cue.
    pub literal_query_consent: ExportConsentClass,
    /// True when the export retains literal query text (only a local replay).
    pub literal_query_text_included: bool,
    /// True when the export leaves the device under this class.
    pub leaves_device: bool,
    /// Query-session ref the cue replays.
    pub query_session_id_ref: String,
    /// Snapshot truth rendered on the cue; never a live rerun.
    pub snapshot_truth: SearchExportSnapshotTruth,
    /// Live-vs-captured semantics rendered on the cue.
    pub result_semantics: SearchResultSemantics,
    /// Captured-vs-current scope honesty state.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// True when reopening requires a rerun before claiming current truth.
    pub rerun_required_for_current_truth: bool,
    /// Always `false`: the cue never claims live current results.
    pub claims_live_current_results: bool,
    /// Rows visible in the captured pass.
    pub visible_rows: u64,
    /// Rows included in the export.
    pub included_rows: u64,
    /// Rows omitted from the export.
    pub omitted_result_count: u64,
    /// Omitted/truncated disclosure flags preserved verbatim.
    pub omitted_or_truncated_flags: Vec<String>,
    /// Deterministic query hash, when retained; never the raw literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// Replay-safety disclosure shown on the cue.
    pub replay_disclosure: String,
}

impl SearchExportReplayCue {
    /// True when the cue is privacy-safe: a cue that leaves the device carries no
    /// literal query text, and the cue type never surfaces the raw literal.
    pub fn is_privacy_safe(&self) -> bool {
        !self.scenario.trim().is_empty()
            && !(self.leaves_device && self.literal_query_text_included)
    }
}

/// Desktop projection of the search-export governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportGovernanceProjectionSet {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet id the desktop ingests verbatim.
    pub ingested_packet_id: String,
    /// Replay cues, in substrate order.
    pub replay_cues: Vec<SearchExportReplayCue>,
}

impl SearchExportGovernanceProjectionSet {
    /// Returns the replay cue for one row, if present.
    pub fn replay_cue(&self, row_id: &str) -> Option<&SearchExportReplayCue> {
        self.replay_cues.iter().find(|cue| cue.row_id == row_id)
    }

    /// True when every cue is privacy-safe and never claims live current results.
    pub fn reuses_substrate(&self) -> bool {
        !self.replay_cues.is_empty()
            && self
                .replay_cues
                .iter()
                .all(SearchExportReplayCue::is_privacy_safe)
            && self
                .replay_cues
                .iter()
                .all(|cue| !cue.claims_live_current_results)
    }
}

/// Projects the desktop search-export replay cues from the packet.
///
/// The projection reuses the canonical export packets verbatim and mints no new
/// identity; it never surfaces literal query text, so the desktop chrome shares
/// one truth with the CLI/headless, support-export, and managed-analytics
/// consumers.
pub fn project_search_export_governance(
    packet: &SearchExportGovernancePacket,
) -> SearchExportGovernanceProjectionSet {
    let replay_cues = packet
        .export_rows
        .iter()
        .map(|row| SearchExportReplayCue {
            row_id: row.row_id.clone(),
            scenario: row.scenario.clone(),
            export_class: row.export_class,
            redaction_state: row.export_packet.redaction_state,
            literal_query_consent: row.literal_query_consent,
            literal_query_text_included: row.literal_query_text_included,
            leaves_device: row.export_class.leaves_device(),
            query_session_id_ref: row.export_packet.query_session_id_ref.clone(),
            snapshot_truth: row.export_packet.snapshot_truth,
            result_semantics: row.replay_safety.result_semantics,
            scope_honesty_state: row.replay_safety.scope_honesty_state,
            rerun_required_for_current_truth: row.replay_safety.rerun_required_for_current_truth,
            claims_live_current_results: row.replay_safety.claims_live_current_results,
            visible_rows: row.export_packet.count_summary.visible_rows,
            included_rows: row.export_packet.count_summary.included_rows,
            omitted_result_count: row.export_packet.count_summary.omitted_result_count,
            omitted_or_truncated_flags: row.export_packet.omitted_or_truncated_flags.clone(),
            query_hash: row.export_packet.query_hash.clone(),
            replay_disclosure: row.replay_safety.disclosure.clone(),
        })
        .collect();

    SearchExportGovernanceProjectionSet {
        record_kind: SEARCH_EXPORT_GOVERNANCE_PROJECTION_SET_RECORD_KIND.to_string(),
        schema_version: SEARCH_EXPORT_GOVERNANCE_PROJECTION_SET_SCHEMA_VERSION,
        ingested_packet_id: packet.packet_id.clone(),
        replay_cues,
    }
}

/// True when the packet names the desktop a first consumer that reuses the same
/// export packets without widening retention or surfacing literal query text.
pub fn shell_is_export_governance_first_consumer(packet: &SearchExportGovernancePacket) -> bool {
    packet.consumer_projections.iter().any(|projection| {
        projection.consumer == SearchExportConsumerClass::DesktopShell
            && projection.reuses_same_export_packets
            && projection.preserves_redaction_mode
            && projection.preserves_count_and_omission_disclosure
            && projection.preserves_replay_safety
            && projection.literal_query_text_excluded
            && projection.ambient_authority_excluded
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_search::seeded_search_export_governance_packet;

    #[test]
    fn projects_a_cue_for_every_export_row() {
        let packet = seeded_search_export_governance_packet();
        let set = project_search_export_governance(&packet);
        assert_eq!(
            set.record_kind,
            SEARCH_EXPORT_GOVERNANCE_PROJECTION_SET_RECORD_KIND
        );
        assert_eq!(set.ingested_packet_id, packet.packet_id);
        assert_eq!(set.replay_cues.len(), packet.export_rows.len());
        assert!(set.reuses_substrate());
        assert!(shell_is_export_governance_first_consumer(&packet));
    }

    #[test]
    fn cues_never_surface_literal_query_text_that_leaves_the_device() {
        let packet = seeded_search_export_governance_packet();
        let set = project_search_export_governance(&packet);
        for cue in &set.replay_cues {
            if cue.leaves_device {
                assert!(!cue.literal_query_text_included);
            }
        }
        // The support bundle cue carries a hash and refs but never the literal.
        let bundle = set.replay_cue("export:support-bundle").expect("bundle cue");
        assert!(bundle.query_hash.is_some());
        let json = serde_json::to_string(&set).expect("serialize");
        assert!(
            !json.contains("kind:file flaky"),
            "literal query text leaked into chrome"
        );
    }

    #[test]
    fn every_cue_is_replay_safe() {
        let packet = seeded_search_export_governance_packet();
        let set = project_search_export_governance(&packet);
        for cue in &set.replay_cues {
            assert!(!cue.claims_live_current_results);
            assert_ne!(cue.snapshot_truth, SearchExportSnapshotTruth::LiveRerun);
            assert_ne!(
                cue.result_semantics,
                SearchResultSemantics::CurrentLiveResults
            );
        }
    }

    #[test]
    fn preserves_omission_disclosure() {
        let packet = seeded_search_export_governance_packet();
        let set = project_search_export_governance(&packet);
        let bundle = set.replay_cue("export:support-bundle").expect("bundle cue");
        assert!(bundle.omitted_result_count > 0);
        assert!(bundle
            .omitted_or_truncated_flags
            .contains(&"hidden_by_current_scope".to_string()));
    }

    #[test]
    fn round_trips_through_json() {
        let packet = seeded_search_export_governance_packet();
        let set = project_search_export_governance(&packet);
        let json = serde_json::to_string(&set).expect("projection serializes");
        let round_trip: SearchExportGovernanceProjectionSet =
            serde_json::from_str(&json).expect("projection deserializes");
        assert_eq!(round_trip, set);
    }
}
