//! Start Center and drag/drop projections for reviewed admission packets.
//!
//! This module is the shell consumer of the workspace admission contract. It
//! maps Start Center actions, clone/import sheet form values, and drag/drop
//! payloads onto the same [`aureline_workspace::AdmissionReviewPacket`] shape.

use std::path::Path;

use aureline_workspace::{
    review_drag_drop_admission, review_entry_admission, AdmissionReviewPacket,
    AdmissionReviewRequest, AdmissionSourceSurface, DragDropAdmissionRequest, DragDropPayloadKind,
    EntryVerb, ResultingMode, TargetKind,
};

use super::StartCenterPrimaryActionId;

/// Builds an admission packet for a Start Center primary action.
pub fn admission_packet_for_start_center_action(
    action_id: StartCenterPrimaryActionId,
) -> AdmissionReviewPacket {
    let (entry_verb, target_kind, resulting_mode, target_specifier, destination) =
        start_center_admission_tuple(action_id);
    let mut request = AdmissionReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        entry_verb,
        target_kind,
        resulting_mode,
        target_specifier,
    );
    if let Some(destination) = destination {
        request = request.with_destination(destination);
    }
    review_entry_admission(request)
}

/// Builds an admission packet for a resolved entry-flow sheet.
pub fn admission_packet_for_resolved_entry(
    source_surface: AdmissionSourceSurface,
    entry_verb: EntryVerb,
    target_kind: TargetKind,
    resulting_mode: ResultingMode,
    target_specifier: impl Into<String>,
    destination: Option<String>,
) -> AdmissionReviewPacket {
    let mut request = AdmissionReviewRequest::new(
        source_surface,
        entry_verb,
        target_kind,
        resulting_mode,
        target_specifier,
    );
    if let Some(destination) = destination {
        request = request.with_destination(destination);
    }
    review_entry_admission(request)
}

/// Builds a clone admission packet from clone sheet form values.
pub fn clone_form_admission_packet(
    remote_url: impl Into<String>,
    destination_path: impl Into<String>,
) -> AdmissionReviewPacket {
    review_entry_admission(
        AdmissionReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            remote_url,
        )
        .with_destination(destination_path),
    )
}

/// Builds an import admission packet from import sheet form values.
pub fn import_form_admission_packet(
    source_path: impl Into<String>,
    destination_workspace_target: impl Into<String>,
) -> AdmissionReviewPacket {
    let source_path = source_path.into();
    let destination_workspace_target = destination_workspace_target.into();
    review_entry_admission(
        AdmissionReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Import,
            classify_import_target(&source_path),
            ResultingMode::ExtractThenReview,
            source_path,
        )
        .with_destination(destination_workspace_target),
    )
}

/// Builds an admission packet for a shell drag/drop payload.
pub fn drag_drop_admission_packet_for_path(
    path: impl AsRef<Path>,
    aggregate_bytes: Option<u64>,
    active_workspace_label: Option<String>,
) -> AdmissionReviewPacket {
    let path = path.as_ref();
    let payload_kind = drag_drop_payload_kind_for_path(path);
    let mut request = DragDropAdmissionRequest::new(payload_kind, path.display().to_string());
    if let Some(bytes) = aggregate_bytes {
        request = request.with_aggregate_bytes(bytes);
    }
    if let Some(active_workspace_label) = active_workspace_label {
        request = request.with_active_workspace(active_workspace_label);
    }
    review_drag_drop_admission(request)
}

/// Renders a packet into compact text rows for the entry sheet.
pub fn compact_admission_review_lines(packet: &AdmissionReviewPacket) -> Vec<String> {
    packet.compact_lines()
}

fn start_center_admission_tuple(
    action_id: StartCenterPrimaryActionId,
) -> (
    EntryVerb,
    TargetKind,
    ResultingMode,
    &'static str,
    Option<&'static str>,
) {
    match action_id {
        StartCenterPrimaryActionId::OpenFolder => (
            EntryVerb::Open,
            TargetKind::LocalFolder,
            ResultingMode::Folder,
            "folder selected from Start Center",
            None,
        ),
        StartCenterPrimaryActionId::OpenWorkspace => (
            EntryVerb::Open,
            TargetKind::WorkspaceManifest,
            ResultingMode::WorkspaceWithRoots,
            "workspace file selected from Start Center",
            None,
        ),
        StartCenterPrimaryActionId::CloneRepository => (
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            "https://example.invalid/repository.git",
            Some("~/Code/repository"),
        ),
        StartCenterPrimaryActionId::RestoreLastSession => (
            EntryVerb::Restore,
            TargetKind::RecoveryCheckpoint,
            ResultingMode::RestoreLastSession,
            "last session checkpoint",
            None,
        ),
        StartCenterPrimaryActionId::ImportFrom => (
            EntryVerb::Import,
            TargetKind::CompetitorConfigRoot,
            ResultingMode::ExtractThenReview,
            "import source selected from Start Center",
            Some("labelled import staging"),
        ),
    }
}

fn classify_import_target(source_path: &str) -> TargetKind {
    let lower = source_path.to_ascii_lowercase();
    if lower.ends_with(".zip")
        || lower.ends_with(".tar")
        || lower.ends_with(".tar.gz")
        || lower.ends_with(".tgz")
    {
        TargetKind::PortableStatePackage
    } else if lower.ends_with(".patch") || lower.ends_with(".diff") {
        TargetKind::HandoffPacket
    } else {
        TargetKind::CompetitorConfigRoot
    }
}

fn drag_drop_payload_kind_for_path(path: &Path) -> DragDropPayloadKind {
    if path.is_dir() {
        if path.join(".git").exists() {
            return DragDropPayloadKind::Repository;
        }
        return DragDropPayloadKind::Folder;
    }
    let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
        return DragDropPayloadKind::File;
    };
    match extension.to_ascii_lowercase().as_str() {
        "code-workspace" | "aureline-workspace" => DragDropPayloadKind::WorkspaceFile,
        "patch" | "diff" => DragDropPayloadKind::Patch,
        "zip" | "tar" | "tgz" | "gz" => DragDropPayloadKind::Archive,
        _ => DragDropPayloadKind::File,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::{AdmissionAction, DestinationDisposition, TransferProgressClass};

    #[test]
    fn start_center_clone_packet_discloses_destination_write_scope() {
        let packet =
            admission_packet_for_start_center_action(StartCenterPrimaryActionId::CloneRepository);
        assert_eq!(packet.entry_verb, EntryVerb::Clone);
        assert_eq!(
            packet.destination_review.disposition,
            DestinationDisposition::WriteToUserDestination
        );
        assert!(packet.clone_review.is_some());
        assert!(packet.trust_and_setup_review.no_silent_trust_grant);
    }

    #[test]
    fn drag_drop_archive_packet_uses_import_verb_and_durable_progress() {
        let packet = drag_drop_admission_packet_for_path(
            "/tmp/archive.zip",
            Some(32 * 1024 * 1024),
            Some("workspace:active".to_string()),
        );
        assert_eq!(packet.entry_verb, EntryVerb::Import);
        let drop = packet.drag_drop_review.as_ref().expect("drop review");
        assert_eq!(drop.advertised_verb, AdmissionAction::Import);
        assert_eq!(
            drop.progress_class,
            TransferProgressClass::DurableProgressWithCancel
        );
        assert!(drop.uses_same_admission_model);
    }

    #[test]
    fn clone_form_packet_preserves_collision_choices() {
        let temp = std::env::temp_dir().join("aureline-shell-admission-collision");
        let _ = std::fs::remove_dir_all(&temp);
        std::fs::create_dir_all(&temp).expect("temp dir");
        std::fs::write(temp.join("README.md"), "occupied\n").expect("seed file");

        let packet = clone_form_admission_packet(
            "https://user:secret@example.com/acme/app.git",
            temp.display().to_string(),
        );
        let collision = packet.collision_review.as_ref().expect("collision");
        assert!(collision
            .safe_actions
            .contains(&AdmissionAction::CloneElsewhere));
        assert!(packet
            .clone_review
            .as_ref()
            .expect("clone")
            .normalized_remote_label
            .contains("example.com/acme/app"));
    }
}
