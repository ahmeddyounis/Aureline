//! Desktop saved-query, history, and deep-link chrome bound to the saved-query
//! governance packet.
//!
//! This module is the desktop first consumer of
//! [`SavedQueryGovernancePacket`](aureline_search::SavedQueryGovernancePacket). It
//! projects compact, inspectable cues for the saved-query list, the query
//! history lane, and the share/deep-link sheet, reusing the governed artifacts
//! verbatim so the chrome renders one truth with the sync/portability and
//! support-export consumers.
//!
//! The projection is privacy-safe by construction: a cue never carries raw query
//! text. It surfaces the privacy and sync class, the captured-vs-current scope
//! disclosure, and — for a shared deep link — the disclosed intent, completeness
//! note, freshness, and whether the link's content signature still verifies, so
//! the chrome can refuse to reopen a tampered or authority-widening link instead
//! of silently following it.

use aureline_search::{
    DeepLinkSignatureScheme, GovernanceConsumerClass, QueryDataClass, SavedQueryGovernancePacket,
    SavedQueryPrivacyClass, SearchRedactionProfile, SearchResultSemantics, SearchRetentionMode,
    SearchScopeHonestyState, SearchSyncClass,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SavedQueryGovernanceProjectionSet`].
pub const SAVED_QUERY_GOVERNANCE_PROJECTION_SET_RECORD_KIND: &str =
    "saved_query_governance_projection_set";

/// Schema version for [`SavedQueryGovernanceProjectionSet`].
pub const SAVED_QUERY_GOVERNANCE_PROJECTION_SET_SCHEMA_VERSION: u32 = 1;

/// One desktop saved-query cue bound to a single governed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryCue {
    /// Row id reused from the substrate.
    pub row_id: String,
    /// Reviewable display name (never raw query text).
    pub display_name: String,
    /// Privacy class shown on the saved-query chip.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Sync class shown on the saved-query chip.
    pub sync_class: SearchSyncClass,
    /// Retention mode shown on the saved-query detail.
    pub retention_mode: SearchRetentionMode,
    /// Captured scope chip label.
    pub scope_label: String,
    /// Scope honesty state rendered on reopen.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// Live-vs-captured semantics rendered on reopen.
    pub result_semantics: SearchResultSemantics,
    /// True when reopening requires a rerun before claiming current truth.
    pub reopen_requires_rerun: bool,
    /// True when the saved query survives reopen, migration, and scope drift.
    pub survives_reopen_migration_and_drift: bool,
    /// User-visible scope drift disclosure.
    pub scope_drift_disclosure: String,
    /// Deterministic query hash, when retained; never the raw literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// True when a raw literal is held in local-only storage (never surfaced).
    pub raw_query_text_held_locally: bool,
}

impl SavedQueryCue {
    /// True when the cue is attributable: the cue type carries no raw query text
    /// field, so privacy safety reduces to keeping a reviewable display name.
    pub fn is_privacy_safe(&self) -> bool {
        !self.display_name.trim().is_empty()
    }
}

/// One desktop signed-deep-link cue for the share/open sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedDeepLinkCue {
    /// Signed-link id reused from the substrate.
    pub signed_link_id: String,
    /// Disclosed intent the link reopens.
    pub intent_summary: String,
    /// Completeness and partiality note shown on the open sheet.
    pub completeness_note: String,
    /// Freshness disclosure shown on the open sheet.
    pub freshness_disclosure: SearchResultSemantics,
    /// Scope honesty state disclosed to the recipient.
    pub scope_disclosure: SearchScopeHonestyState,
    /// Supportable return path the recipient returns focus to.
    pub return_anchor_ref: String,
    /// Trust origin backing the content signature.
    pub signature_scheme: DeepLinkSignatureScheme,
    /// True when the carried content signature still verifies the disclosure.
    pub signature_verifies: bool,
    /// Always `false`: the cue never implies live current certainty.
    pub implies_live_current_certainty: bool,
    /// True when the chrome may safely reopen the link.
    pub safe_to_reopen: bool,
}

/// One desktop retention cue for the portability/privacy panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionCue {
    /// Data class governed by the row.
    pub data_class: QueryDataClass,
    /// Local retention mode.
    pub local_retention_mode: SearchRetentionMode,
    /// True only when the data class syncs without explicit opt-in.
    pub synced_by_default: bool,
    /// Redaction applied before any sync, share, or export.
    pub on_sync_redaction: SearchRedactionProfile,
    /// User-visible disclosure of the retention posture.
    pub disclosure: String,
}

/// Desktop projection of the saved-query governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryGovernanceProjectionSet {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet id the desktop ingests verbatim.
    pub ingested_packet_id: String,
    /// Saved-query cues, in substrate order.
    pub saved_query_cues: Vec<SavedQueryCue>,
    /// Signed deep-link cues, in substrate order.
    pub signed_deep_link_cues: Vec<SignedDeepLinkCue>,
    /// Retention cues, in substrate order.
    pub retention_cues: Vec<RetentionCue>,
}

impl SavedQueryGovernanceProjectionSet {
    /// Returns the saved-query cue for one row, if present.
    pub fn saved_query_cue(&self, row_id: &str) -> Option<&SavedQueryCue> {
        self.saved_query_cues
            .iter()
            .find(|cue| cue.row_id == row_id)
    }

    /// True when every cue is privacy-safe and every signed link cue carries a
    /// verifiable, intent-only disclosure.
    pub fn reuses_substrate(&self) -> bool {
        !self.saved_query_cues.is_empty()
            && self
                .saved_query_cues
                .iter()
                .all(SavedQueryCue::is_privacy_safe)
            && self
                .signed_deep_link_cues
                .iter()
                .all(|cue| !cue.implies_live_current_certainty && !cue.return_anchor_ref.is_empty())
    }
}

/// Projects the desktop saved-query governance cues from the packet.
///
/// The projection reuses the durable artifacts verbatim and mints no new
/// identity; it never surfaces raw query text, so the desktop chrome shares one
/// truth with the sync/portability and support-export consumers.
pub fn project_saved_query_governance(
    packet: &SavedQueryGovernancePacket,
) -> SavedQueryGovernanceProjectionSet {
    let saved_query_cues = packet
        .saved_queries
        .iter()
        .map(|row| SavedQueryCue {
            row_id: row.row_id.clone(),
            display_name: row.saved_query.display_name.clone(),
            privacy_class: row.saved_query.privacy_class,
            sync_class: row.saved_query.sync_class,
            retention_mode: row.saved_query.retention_mode,
            scope_label: row.saved_query.scope_label.clone(),
            scope_honesty_state: row.saved_query.scope_honesty_state,
            result_semantics: row.saved_query.result_semantics,
            reopen_requires_rerun: row.scope_drift.rerun_required,
            survives_reopen_migration_and_drift: row.survives_reopen
                && row.survives_migration
                && row.survives_scope_drift,
            scope_drift_disclosure: row.scope_drift.disclosure.clone(),
            query_hash: row.saved_query.query_hash.clone(),
            raw_query_text_held_locally: row.saved_query.query_text.is_some(),
        })
        .collect();

    let signed_deep_link_cues = packet
        .signed_deep_links
        .iter()
        .map(|link| {
            let signature_verifies = link.signature_verifies();
            SignedDeepLinkCue {
                signed_link_id: link.signed_link_id.clone(),
                intent_summary: link.intent_summary.clone(),
                completeness_note: link.completeness_note.clone(),
                freshness_disclosure: link.freshness_disclosure,
                scope_disclosure: link.scope_disclosure,
                return_anchor_ref: link.return_anchor_ref.clone(),
                signature_scheme: link.signature_scheme,
                signature_verifies,
                implies_live_current_certainty: link.implies_live_current_certainty,
                safe_to_reopen: signature_verifies
                    && !link.implies_live_current_certainty
                    && !link.deep_link.access_widening_allowed,
            }
        })
        .collect();

    let retention_cues = packet
        .retention_matrix
        .iter()
        .map(|row| RetentionCue {
            data_class: row.data_class,
            local_retention_mode: row.local_retention_mode,
            synced_by_default: row.synced_by_default,
            on_sync_redaction: row.on_sync_redaction,
            disclosure: row.disclosure.clone(),
        })
        .collect();

    SavedQueryGovernanceProjectionSet {
        record_kind: SAVED_QUERY_GOVERNANCE_PROJECTION_SET_RECORD_KIND.to_string(),
        schema_version: SAVED_QUERY_GOVERNANCE_PROJECTION_SET_SCHEMA_VERSION,
        ingested_packet_id: packet.packet_id.clone(),
        saved_query_cues,
        signed_deep_link_cues,
        retention_cues,
    }
}

/// True when the packet names the desktop a first consumer that reuses the same
/// governed artifacts without widening authority or surfacing raw query text.
pub fn shell_is_governance_first_consumer(packet: &SavedQueryGovernancePacket) -> bool {
    packet.consumer_projections.iter().any(|projection| {
        projection.consumer == GovernanceConsumerClass::ProductUi
            && projection.reuses_same_artifacts
            && projection.preserves_privacy_and_sync_class
            && projection.preserves_captured_vs_current_scope
            && projection.raw_query_text_excluded
            && !projection.widens_authority
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_search::seeded_saved_query_governance_packet;

    #[test]
    fn projects_a_cue_for_every_governed_row() {
        let packet = seeded_saved_query_governance_packet();
        let set = project_saved_query_governance(&packet);
        assert_eq!(
            set.record_kind,
            SAVED_QUERY_GOVERNANCE_PROJECTION_SET_RECORD_KIND
        );
        assert_eq!(set.ingested_packet_id, packet.packet_id);
        assert_eq!(set.saved_query_cues.len(), packet.saved_queries.len());
        assert_eq!(
            set.signed_deep_link_cues.len(),
            packet.signed_deep_links.len()
        );
        assert_eq!(set.retention_cues.len(), packet.retention_matrix.len());
        assert!(set.reuses_substrate());
        assert!(shell_is_governance_first_consumer(&packet));
    }

    #[test]
    fn cues_never_surface_raw_query_text() {
        let packet = seeded_saved_query_governance_packet();
        let set = project_saved_query_governance(&packet);
        // The local-only row holds a literal locally, but the cue only flags it.
        let local = set
            .saved_query_cue("saved-query:local-private")
            .expect("local-private cue");
        assert!(local.raw_query_text_held_locally);
        let json = serde_json::to_string(&set).expect("serialize");
        assert!(
            !json.contains("retry budget"),
            "raw query text leaked into chrome"
        );
    }

    #[test]
    fn signed_link_cues_are_safe_only_when_they_verify() {
        let packet = seeded_saved_query_governance_packet();
        let set = project_saved_query_governance(&packet);
        for cue in &set.signed_deep_link_cues {
            assert!(cue.signature_verifies);
            assert!(cue.safe_to_reopen);
            assert!(!cue.implies_live_current_certainty);
            assert!(!cue.return_anchor_ref.is_empty());
        }
    }

    #[test]
    fn tampered_link_is_not_safe_to_reopen() {
        let mut packet = seeded_saved_query_governance_packet();
        packet.signed_deep_links[0].completeness_note = "All results are current.".to_string();
        let set = project_saved_query_governance(&packet);
        let cue = &set.signed_deep_link_cues[0];
        assert!(!cue.signature_verifies);
        assert!(!cue.safe_to_reopen);
    }

    #[test]
    fn round_trips_through_json() {
        let packet = seeded_saved_query_governance_packet();
        let set = project_saved_query_governance(&packet);
        let json = serde_json::to_string(&set).expect("projection serializes");
        let round_trip: SavedQueryGovernanceProjectionSet =
            serde_json::from_str(&json).expect("projection deserializes");
        assert_eq!(round_trip, set);
    }
}
