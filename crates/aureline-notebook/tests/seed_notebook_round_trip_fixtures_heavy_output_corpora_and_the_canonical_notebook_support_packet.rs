use aureline_notebook::{
    current_notebook_support_packet, HeavyOutputCorpusEntry, HeavyOutputCorpusSizeBucketClass,
    HeavyOutputCorpusTrustImplicationClass, HeavyOutputCorpusVirtualizationClass,
    NotebookRoundTripFixture, NotebookRoundTripFixtureKindClass, NotebookSupportPacket,
    NotebookSupportPacketCoverageClass, HEAVY_OUTPUT_CORPUS_ENTRY_RECORD_KIND,
    NOTEBOOK_ROUND_TRIP_FIXTURE_RECORD_KIND, NOTEBOOK_SUPPORT_PACKET_RECORD_KIND,
    NOTEBOOK_SUPPORT_SCHEMA_VERSION,
};

fn sample_round_trip_fixture() -> NotebookRoundTripFixture {
    NotebookRoundTripFixture {
        record_kind: NOTEBOOK_ROUND_TRIP_FIXTURE_RECORD_KIND.to_owned(),
        notebook_support_schema_version: NOTEBOOK_SUPPORT_SCHEMA_VERSION,
        fixture_id: "nb.int.fixture.01".to_owned(),
        document_id_ref: "nb.int.doc.01".to_owned(),
        fixture_kind_class: NotebookRoundTripFixtureKindClass::NoKernelEditable,
        assertion_kind_class_refs: vec!["source_survives".to_owned()],
        expected_result_class: "pass".to_owned(),
        loss_summary: None,
        summary: "Integration test round-trip fixture.".to_owned(),
    }
}

fn sample_heavy_output_corpus_entry() -> HeavyOutputCorpusEntry {
    HeavyOutputCorpusEntry {
        record_kind: HEAVY_OUTPUT_CORPUS_ENTRY_RECORD_KIND.to_owned(),
        notebook_support_schema_version: NOTEBOOK_SUPPORT_SCHEMA_VERSION,
        corpus_entry_id: "nb.int.corpus.01".to_owned(),
        document_id_ref: "nb.int.doc.01".to_owned(),
        size_bucket_class: HeavyOutputCorpusSizeBucketClass::Large,
        output_count: 48,
        contains_rich_output: false,
        trust_implication_class: HeavyOutputCorpusTrustImplicationClass::Sandboxed,
        virtualization_class: HeavyOutputCorpusVirtualizationClass::Externalized,
        summary: "Integration test heavy-output corpus entry.".to_owned(),
    }
}

#[test]
fn integration_round_trip_fixture_validates_clean() {
    let fixture = sample_round_trip_fixture();
    assert!(
        fixture.validate().is_empty(),
        "integration round-trip fixture should be clean: {:?}",
        fixture.validate()
    );
}

#[test]
fn integration_heavy_output_corpus_entry_validates_clean() {
    let entry = sample_heavy_output_corpus_entry();
    assert!(
        entry.validate().is_empty(),
        "integration heavy-output corpus entry should be clean: {:?}",
        entry.validate()
    );
}

#[test]
fn integration_packet_validates_clean() {
    let packet = NotebookSupportPacket {
        schema_version: NOTEBOOK_SUPPORT_SCHEMA_VERSION,
        record_kind: NOTEBOOK_SUPPORT_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.int.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        fixture_kind_classes: NotebookRoundTripFixtureKindClass::ALL.to_vec(),
        size_bucket_classes: HeavyOutputCorpusSizeBucketClass::ALL.to_vec(),
        trust_implication_classes: HeavyOutputCorpusTrustImplicationClass::ALL.to_vec(),
        virtualization_classes: HeavyOutputCorpusVirtualizationClass::ALL.to_vec(),
        coverage_classes: NotebookSupportPacketCoverageClass::ALL.to_vec(),
        example_round_trip_fixtures: vec![sample_round_trip_fixture()],
        example_heavy_output_corpus_entries: vec![sample_heavy_output_corpus_entry()],
        summary: "Integration test notebook support packet.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "integration packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn integration_embedded_packet_parses_and_validates() {
    let packet = current_notebook_support_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_SUPPORT_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_SUPPORT_PACKET_RECORD_KIND);
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet must validate cleanly: {:?}",
        findings
    );
}

#[test]
fn integration_round_trip_fixture_rejects_bad_record_kind() {
    let mut fixture = sample_round_trip_fixture();
    fixture.record_kind = "wrong_kind".to_owned();
    let findings = fixture.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_round_trip_fixture.record_kind"));
}

#[test]
fn integration_heavy_output_corpus_entry_rejects_bad_schema_version() {
    let mut entry = sample_heavy_output_corpus_entry();
    entry.notebook_support_schema_version = 999;
    let findings = entry.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "heavy_output_corpus_entry.schema_version"));
}

#[test]
fn integration_round_trip_fixture_rejects_pass_with_loss_summary() {
    let mut fixture = sample_round_trip_fixture();
    fixture.loss_summary = Some("should not be here".to_owned());
    let findings = fixture.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_round_trip_fixture.loss_summary_not_allowed"));
}
