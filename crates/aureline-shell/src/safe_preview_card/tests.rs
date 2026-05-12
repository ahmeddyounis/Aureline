use super::*;
use aureline_content_safety::{detect_suspicious_content, TrustClass};
use aureline_preview::{
    build_generated_content_preview, build_oversized_artifact_preview, build_risky_text_preview,
    GeneratedContentInput, OversizedArtifactInput, RiskyTextInput,
};

fn risky_record() -> aureline_preview::SafePreviewRecord {
    let text = "let \u{202E}admin\u{200D}user = 1;";
    build_risky_text_preview(RiskyTextInput {
        preview_id: "preview:risky_text:src/lib.rs#config_loader".to_owned(),
        source_subject_ref: "buffer:file:src/lib.rs#identifier:config_loader".to_owned(),
        source_surface_family: "editor".to_owned(),
        trust_class: TrustClass::RawText,
        detection: detect_suspicious_content(text),
    })
}

fn oversized_record() -> aureline_preview::SafePreviewRecord {
    build_oversized_artifact_preview(OversizedArtifactInput {
        preview_id: "preview:oversized:log:tail.session".to_owned(),
        source_subject_ref: "capture:terminal_log:tail.session".to_owned(),
        source_surface_family: "output_viewer".to_owned(),
        trust_class: TrustClass::SanitizedRich,
        total_byte_count: 12_500_000,
        visible_byte_count: 64_000,
        visible_line_count: 1_024,
        omitted_bytes_estimate: 12_436_000,
        omitted_line_count_estimate: 199_232,
    })
}

fn generated_record() -> aureline_preview::SafePreviewRecord {
    build_generated_content_preview(GeneratedContentInput {
        preview_id: "preview:generated:ai_change_summary:001".to_owned(),
        source_subject_ref: "ai:change_summary:proposal:001".to_owned(),
        source_surface_family: "rich_preview".to_owned(),
        generator_id: "composer-seed/m1".to_owned(),
        citation_anchor_refs: vec!["anchor:src/router.rs#fn:dispatch:lines:42-58".to_owned()],
        canonical_source_subject_ref: Some("buffer:file:src/router.rs#fn:dispatch".to_owned()),
    })
}

#[test]
fn protected_walk_risky_text_snapshot_includes_every_section_in_canonical_order() {
    let snapshot = SafePreviewCardSnapshot::project(&risky_record());
    let section_ids: Vec<SafePreviewSectionId> =
        snapshot.sections.iter().map(|s| s.section_id).collect();
    assert_eq!(
        section_ids,
        vec![
            SafePreviewSectionId::PrototypeLabel,
            SafePreviewSectionId::Header,
            SafePreviewSectionId::CurrentlyVisible,
            SafePreviewSectionId::BodyExtent,
            SafePreviewSectionId::CopyExportOptions,
            SafePreviewSectionId::ClaimLimits,
            SafePreviewSectionId::Invariants,
        ]
    );
    assert_eq!(snapshot.content_class_token, "risky_text");
    assert!(!snapshot.has_invariant_violations);
    assert_eq!(snapshot.copy_export_option_count, 3);
}

#[test]
fn copy_export_section_addresses_each_option_by_id() {
    let snapshot = SafePreviewCardSnapshot::project(&risky_record());
    let section = snapshot
        .section(SafePreviewSectionId::CopyExportOptions)
        .expect("copy_export_options section");
    assert_eq!(section.rows.len(), 3);
    for row in &section.rows {
        match &row.address {
            SafePreviewRowAddress::CopyExportOption {
                option_id,
                action_id,
            } => {
                assert!(!option_id.is_empty());
                assert!(matches!(
                    action_id.as_str(),
                    "copy_raw" | "copy_escaped" | "export_metadata_only"
                ));
            }
            _ => panic!("copy/export row must address an option"),
        }
        assert_eq!(row.status, SafePreviewRowStatus::Action);
    }
}

#[test]
fn oversized_card_renders_body_extent_with_omission_chip() {
    let snapshot = SafePreviewCardSnapshot::project(&oversized_record());
    let section = snapshot
        .section(SafePreviewSectionId::BodyExtent)
        .expect("body_extent section");
    let labels: Vec<&str> = section.rows.iter().map(|r| r.row_id.as_str()).collect();
    assert!(labels.contains(&"total_bytes"));
    assert!(labels.contains(&"visible_bytes"));
    assert!(labels.contains(&"visible_lines"));
    assert!(labels.contains(&"suspicious_finding_count"));
    assert!(!snapshot.has_invariant_violations);
}

#[test]
fn generated_card_pins_currently_visible_to_generated_label() {
    let snapshot = SafePreviewCardSnapshot::project(&generated_record());
    let section = snapshot
        .section(SafePreviewSectionId::CurrentlyVisible)
        .expect("currently_visible section");
    assert_eq!(section.rows[0].value_token.as_deref(), Some("generated"));
    assert_eq!(snapshot.origin_class_token, "generated");
    assert!(!snapshot.has_invariant_violations);
}

#[test]
fn invariants_section_is_blocked_when_record_is_dishonest() {
    let mut record = risky_record();
    // Drop the paired copy_escaped action — the named failure drill where
    // raw and escaped lose their pairing.
    record
        .copy_export_options
        .retain(|opt| opt.action_id != "copy_escaped");
    let snapshot = SafePreviewCardSnapshot::project(&record);
    assert!(snapshot.has_invariant_violations);
    let section = snapshot
        .section(SafePreviewSectionId::Invariants)
        .expect("invariants section");
    assert!(section
        .rows
        .iter()
        .all(|row| row.status == SafePreviewRowStatus::Blocked));
    let tokens: Vec<String> = section
        .rows
        .iter()
        .filter_map(|row| row.value_token.clone())
        .collect();
    assert!(tokens.iter().any(|t| t == "missing_paired_action"));
}

#[test]
fn render_plaintext_quotes_every_section_heading() {
    let snapshot = SafePreviewCardSnapshot::project(&risky_record());
    let text = snapshot.render_plaintext();
    for section in &snapshot.sections {
        assert!(
            text.contains(section.heading.as_str()),
            "missing {}",
            section.heading
        );
    }
    assert!(text.contains("m1_prototype_safe_preview_and_copy_export"));
    assert!(text.contains("risky_text"));
    assert!(
        text.contains("currently_visible=escaped") || text.contains("Currently visible: escaped")
    );
}

#[test]
fn snapshot_round_trips_through_serde_json() {
    let snapshot = SafePreviewCardSnapshot::project(&oversized_record());
    let json = serde_json::to_string(&snapshot).expect("serialize");
    let back: SafePreviewCardSnapshot = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.preview_id, snapshot.preview_id);
    assert_eq!(back.sections.len(), snapshot.sections.len());
    assert_eq!(
        back.has_invariant_violations,
        snapshot.has_invariant_violations
    );
}
