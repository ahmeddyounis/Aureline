//! Support-export projection for entry interstitials.
//!
//! The same interstitial packet shown to the user is carried into the support
//! export so a route/origin incident can be reconstructed from the export
//! without scraping transient UI text. The packet is metadata-safe: it carries
//! the typed classes, the opaque object-identity ref, and the redaction-safe
//! labels the record already holds — never raw URLs, raw paths, raw callback
//! bodies, or credentials.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{BoundaryClass, EntryInterstitialRecord, ENTRY_INTERSTITIAL_SCHEMA_VERSION};

/// Stable record-kind tag for [`EntryInterstitialSupportPacket`].
pub const ENTRY_INTERSTITIAL_SUPPORT_PACKET_RECORD_KIND: &str =
    "entry_interstitial_support_packet_record";

/// Metadata-safe support-export wrapper around an interstitial record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryInterstitialSupportPacket {
    pub record_kind: String,
    pub entry_interstitial_schema_version: u32,
    pub interstitial_id: String,
    /// Entry kind token (e.g. `protocol_deep_link`).
    pub kind: String,
    /// Source class token (e.g. `system_default_browser`).
    pub source_class: String,
    /// Redaction-safe source label.
    pub source_label: String,
    /// Requested action class token.
    pub requested_action: String,
    /// Boundary class tokens, in stable order.
    pub boundary_classes: Vec<String>,
    /// Authority effect class token.
    pub authority_effect: String,
    /// Target kind token.
    pub target_kind: String,
    /// Opaque, log-safe object identity.
    pub object_identity_ref: String,
    /// Target truth-state token.
    pub target_truth_state: String,
    /// True when a truthful placeholder was shown instead of an exact open.
    pub placeholder_shown: bool,
    /// The canonical in-product command the confirm action runs.
    pub canonical_command_ref: String,
    /// Invariant echoes carried verbatim for incident reconstruction.
    pub silent_execution_forbidden: bool,
    pub authority_not_widened: bool,
    pub reopens_generic_home: bool,
    /// Reviewer-facing summary line copied from the record.
    pub summary_line: String,
}

impl EntryInterstitialSupportPacket {
    /// Projects a metadata-safe support packet from an interstitial record.
    pub fn from_record(record: &EntryInterstitialRecord) -> Self {
        Self {
            record_kind: ENTRY_INTERSTITIAL_SUPPORT_PACKET_RECORD_KIND.to_string(),
            entry_interstitial_schema_version: ENTRY_INTERSTITIAL_SCHEMA_VERSION,
            interstitial_id: record.interstitial_id.clone(),
            kind: record.kind.as_str().to_string(),
            source_class: record.source_class.as_str().to_string(),
            source_label: record.source_label.clone(),
            requested_action: record.requested_action.as_str().to_string(),
            boundary_classes: boundary_tokens(&record.boundary_classes),
            authority_effect: record.authority_effect.as_str().to_string(),
            target_kind: record.target_scope.target_kind.as_str().to_string(),
            object_identity_ref: record.target_scope.object_identity_ref.clone(),
            target_truth_state: record.target_scope.truth_state.as_str().to_string(),
            placeholder_shown: record.target_placeholder.is_some(),
            canonical_command_ref: record.canonical_command_ref.clone(),
            silent_execution_forbidden: record.silent_execution_forbidden,
            authority_not_widened: record.authority_not_widened,
            reopens_generic_home: record.reopens_generic_home,
            summary_line: record.summary_line.clone(),
        }
    }
}

fn boundary_tokens(classes: &[BoundaryClass]) -> Vec<String> {
    classes.iter().map(|b| b.as_str().to_string()).collect()
}

/// Writes a support packet to
/// `<export_root>/entry_interstitial_support_packet.json`.
pub fn write_entry_interstitial_support_packet(
    export_root: &Path,
    packet: &EntryInterstitialSupportPacket,
) -> Result<(), String> {
    std::fs::create_dir_all(export_root)
        .map_err(|err| format!("create export root failed: {err}"))?;
    let path = export_root.join("entry_interstitial_support_packet.json");
    let json = serde_json::to_string_pretty(packet)
        .map_err(|err| format!("serialize support packet failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry_interstitials::{
        evaluate_entry_interstitial, AuthorityEffectClass, EntryInterstitialKind,
        EntryInterstitialRequest, EntrySourceClass, EntryTargetKind, RequestedActionClass,
        TargetScope, TargetTruthState,
    };

    fn managed_resume_request() -> EntryInterstitialRequest {
        EntryInterstitialRequest {
            interstitial_id: "ei:test:managed:01".to_string(),
            kind: EntryInterstitialKind::ManagedResume,
            source_class: EntrySourceClass::ManagedAdminSurface,
            source_label: "Managed admin surface".to_string(),
            requested_action: RequestedActionClass::ManagedResume,
            target: TargetScope {
                target_kind: EntryTargetKind::ManagedWorkspace,
                object_identity_ref: "obj:managed-ws:99".to_string(),
                target_label: "Acme managed workspace".to_string(),
                workspace_scope_label: "Acme / platform".to_string(),
                tenant_scope_label: Some("Acme Corp".to_string()),
                channel_or_build_owner_label: Some("stable / admin-owned".to_string()),
                truth_state: TargetTruthState::ExactAvailable,
                identity_review_required: true,
            },
            authority_effect: AuthorityEffectClass::PolicyBoundaryReview,
            crosses_tenant_boundary: true,
            canonical_command_id: "cmd:workspace.restore_from_checkpoint".to_string(),
        }
    }

    #[test]
    fn support_packet_is_metadata_safe_and_echoes_invariants() {
        let decision = evaluate_entry_interstitial(&managed_resume_request());
        let record = decision.record().expect("interstitial required");
        let packet = EntryInterstitialSupportPacket::from_record(record);

        assert_eq!(
            packet.record_kind,
            ENTRY_INTERSTITIAL_SUPPORT_PACKET_RECORD_KIND
        );
        assert_eq!(packet.kind, "managed_resume");
        assert_eq!(packet.source_class, "managed_admin_surface");
        assert_eq!(packet.object_identity_ref, "obj:managed-ws:99");
        assert_eq!(
            packet.canonical_command_ref,
            "cmd:workspace.restore_from_checkpoint"
        );
        assert!(packet.silent_execution_forbidden);
        assert!(packet.authority_not_widened);
        assert!(!packet.reopens_generic_home);
        assert!(packet
            .boundary_classes
            .contains(&"tenant_boundary".to_string()));
        assert!(packet
            .boundary_classes
            .contains(&"policy_boundary".to_string()));
    }

    #[test]
    fn support_packet_round_trips_via_serde() {
        let decision = evaluate_entry_interstitial(&managed_resume_request());
        let record = decision.record().expect("interstitial required");
        let packet = EntryInterstitialSupportPacket::from_record(record);

        let dir = tempfile::tempdir().expect("tempdir");
        write_entry_interstitial_support_packet(dir.path(), &packet).expect("write");
        let read =
            std::fs::read_to_string(dir.path().join("entry_interstitial_support_packet.json"))
                .expect("read");
        let back: EntryInterstitialSupportPacket = serde_json::from_str(&read).expect("parse");
        assert_eq!(back, packet);
    }
}
