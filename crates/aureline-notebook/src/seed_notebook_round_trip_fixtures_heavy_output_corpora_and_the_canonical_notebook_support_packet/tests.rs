use super::*;

fn sample_round_trip_fixture() -> NotebookRoundTripFixture {
    NotebookRoundTripFixture {
        record_kind: NOTEBOOK_ROUND_TRIP_FIXTURE_RECORD_KIND.to_owned(),
        notebook_support_schema_version: NOTEBOOK_SUPPORT_SCHEMA_VERSION,
        fixture_id: "nb.fixture.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        fixture_kind_class: NotebookRoundTripFixtureKindClass::CleanCanonical,
        assertion_kind_class_refs: vec![
            "metadata_survives".to_owned(),
            "cell_id_survives".to_owned(),
        ],
        expected_result_class: "pass".to_owned(),
        loss_summary: None,
        summary: "Clean canonical notebook round-trip fixture.".to_owned(),
    }
}

fn sample_heavy_output_corpus_entry() -> HeavyOutputCorpusEntry {
    HeavyOutputCorpusEntry {
        record_kind: HEAVY_OUTPUT_CORPUS_ENTRY_RECORD_KIND.to_owned(),
        notebook_support_schema_version: NOTEBOOK_SUPPORT_SCHEMA_VERSION,
        corpus_entry_id: "nb.corpus.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        size_bucket_class: HeavyOutputCorpusSizeBucketClass::Medium,
        output_count: 12,
        contains_rich_output: true,
        trust_implication_class: HeavyOutputCorpusTrustImplicationClass::TrustedVirtualized,
        virtualization_class: HeavyOutputCorpusVirtualizationClass::Paginated,
        summary: "Medium-sized output corpus with rich outputs and pagination.".to_owned(),
    }
}

#[test]
fn round_trip_fixture_validates_clean() {
    let fixture = sample_round_trip_fixture();
    assert!(
        fixture.validate().is_empty(),
        "round-trip fixture should be clean: {:?}",
        fixture.validate()
    );
}

#[test]
fn heavy_output_corpus_entry_validates_clean() {
    let entry = sample_heavy_output_corpus_entry();
    assert!(
        entry.validate().is_empty(),
        "heavy-output corpus entry should be clean: {:?}",
        entry.validate()
    );
}

#[test]
fn round_trip_fixture_requires_loss_summary_on_non_pass() {
    let mut fixture = sample_round_trip_fixture();
    fixture.expected_result_class = "fail".to_owned();
    fixture.loss_summary = None;
    let findings = fixture.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_round_trip_fixture.loss_summary_required"));
}

#[test]
fn round_trip_fixture_rejects_loss_summary_on_pass() {
    let mut fixture = sample_round_trip_fixture();
    fixture.loss_summary = Some("unexpected note".to_owned());
    let findings = fixture.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_round_trip_fixture.loss_summary_not_allowed"));
}

#[test]
fn heavy_output_corpus_entry_rejects_zero_output_count() {
    let mut entry = sample_heavy_output_corpus_entry();
    entry.output_count = 0;
    let findings = entry.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "heavy_output_corpus_entry.output_count"));
}

#[test]
fn heavy_output_corpus_entry_rejects_small_with_virtualization() {
    let mut entry = sample_heavy_output_corpus_entry();
    entry.size_bucket_class = HeavyOutputCorpusSizeBucketClass::Small;
    entry.virtualization_class = HeavyOutputCorpusVirtualizationClass::Paginated;
    let findings = entry.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "heavy_output_corpus_entry.small_no_virtualization"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(
        NotebookRoundTripFixtureKindClass::CleanCanonical.as_str(),
        "clean_canonical"
    );
    assert_eq!(
        NotebookRoundTripFixtureKindClass::UnknownNamespaceDense.as_str(),
        "unknown_namespace_dense"
    );
    assert_eq!(
        NotebookRoundTripFixtureKindClass::ExportOnly.as_str(),
        "export_only"
    );
    assert_eq!(
        HeavyOutputCorpusSizeBucketClass::VeryLarge.as_str(),
        "very_large"
    );
    assert_eq!(
        HeavyOutputCorpusTrustImplicationClass::SanitizedVirtualized.as_str(),
        "sanitized_virtualized"
    );
    assert_eq!(
        HeavyOutputCorpusVirtualizationClass::LazyLoaded.as_str(),
        "lazy_loaded"
    );
    assert_eq!(
        NotebookSupportPacketCoverageClass::FixtureOnly.as_str(),
        "fixture_only"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookSupportPacket {
        schema_version: NOTEBOOK_SUPPORT_SCHEMA_VERSION,
        record_kind: NOTEBOOK_SUPPORT_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.packet.support.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        fixture_kind_classes: NotebookRoundTripFixtureKindClass::ALL.to_vec(),
        size_bucket_classes: HeavyOutputCorpusSizeBucketClass::ALL.to_vec(),
        trust_implication_classes: HeavyOutputCorpusTrustImplicationClass::ALL.to_vec(),
        virtualization_classes: HeavyOutputCorpusVirtualizationClass::ALL.to_vec(),
        coverage_classes: NotebookSupportPacketCoverageClass::ALL.to_vec(),
        example_round_trip_fixtures: vec![sample_round_trip_fixture()],
        example_heavy_output_corpus_entries: vec![sample_heavy_output_corpus_entry()],
        summary: "Notebook support packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_support_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_SUPPORT_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_SUPPORT_PACKET_RECORD_KIND);
}
