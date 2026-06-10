use super::*;

const PACKET_ID: &str = "ai-memory-posture:m5:0001";
const POSTURE_RUN_ID: &str = "ai-memory-posture-run:m5:0001";

fn memory_class_coverage() -> MemoryClassCoverageBlock {
    let class_rows = AiStateClass::required_coverage()
        .into_iter()
        .map(|state_class| {
            // The evidence-governed copy is held beyond a delete request under a
            // disclosed evidence-retention hold; every other class deletes fully.
            let held = matches!(state_class, AiStateClass::RetainedEvidenceCopy);
            MemoryClassCoverageRow {
                state_class,
                delete_fan_out_covered: !held,
                export_fan_out_covered: true,
                retention_hold: if held {
                    RetentionHoldClass::EvidenceHold
                } else {
                    RetentionHoldClass::NoHold
                },
                hold_disclosed: held,
                disclosed: true,
            }
        })
        .collect();
    MemoryClassCoverageBlock {
        coverage_set_id: "memory-class-coverage:m5:0001".to_owned(),
        all_classes_covered: true,
        class_rows,
    }
}

fn explicit_saved_memory() -> ExplicitSavedMemoryBlock {
    ExplicitSavedMemoryBlock {
        saved_set_id: "saved-memory-set:m5:0001".to_owned(),
        owner_accountable: true,
        all_revocable: true,
        saved_rows: vec![
            ExplicitSavedMemoryRow {
                entry_id: "saved:m5:0001:user-pref".to_owned(),
                scope: SavedMemoryScopeClass::UserScoped,
                actor_class: SavedMemoryActorClass::EndUser,
                consent: SavedMemoryConsentClass::ExplicitlyConsented,
                revocable: true,
                disclosed: true,
            },
            ExplicitSavedMemoryRow {
                entry_id: "saved:m5:0001:repo-fact".to_owned(),
                scope: SavedMemoryScopeClass::RepoScoped,
                actor_class: SavedMemoryActorClass::RepoOwner,
                consent: SavedMemoryConsentClass::ExplicitlyConsented,
                revocable: true,
                disclosed: true,
            },
            ExplicitSavedMemoryRow {
                entry_id: "saved:m5:0001:org-policy".to_owned(),
                scope: SavedMemoryScopeClass::OrgScoped,
                actor_class: SavedMemoryActorClass::OrgAdmin,
                consent: SavedMemoryConsentClass::PolicyDefault,
                revocable: true,
                disclosed: true,
            },
        ],
    }
}

fn scoped_operations() -> ScopedDeletionExportBlock {
    ScopedDeletionExportBlock {
        operation_set_id: "scoped-operation-set:m5:0001".to_owned(),
        per_class_fan_out_enforced: true,
        receipts_required: true,
        operation_rows: vec![
            ScopedDeletionExportRow {
                operation_id: "op:m5:0001:workspace-delete".to_owned(),
                operation_kind: MemoryOperationKindClass::Deletion,
                scope: DeletionExportScopeClass::WorkspaceScoped,
                fan_out_completeness: FanOutCompletenessClass::AllClassesCovered,
                receipt_verification: ReceiptVerificationClass::VerifiedReceipt,
                receipt_ref: "receipt:m5:0001:workspace-delete".to_owned(),
                all_classes_addressed: true,
                disclosed: true,
            },
            ScopedDeletionExportRow {
                operation_id: "op:m5:0001:tenant-export".to_owned(),
                operation_kind: MemoryOperationKindClass::Export,
                scope: DeletionExportScopeClass::TenantScoped,
                fan_out_completeness: FanOutCompletenessClass::AllClassesCovered,
                receipt_verification: ReceiptVerificationClass::VerifiedReceipt,
                receipt_ref: "receipt:m5:0001:tenant-export".to_owned(),
                all_classes_addressed: true,
                disclosed: true,
            },
            ScopedDeletionExportRow {
                operation_id: "op:m5:0001:tenant-delete-hold".to_owned(),
                operation_kind: MemoryOperationKindClass::Deletion,
                scope: DeletionExportScopeClass::TenantScoped,
                fan_out_completeness: FanOutCompletenessClass::PartialPendingRetentionHold,
                receipt_verification: ReceiptVerificationClass::PendingVerification,
                receipt_ref: "receipt:m5:0001:tenant-delete-hold".to_owned(),
                all_classes_addressed: false,
                disclosed: true,
            },
        ],
    }
}

fn consumer_surface_parity() -> Vec<MemoryPostureSurfaceParityRow> {
    MemoryPostureConsumerSurface::ALL
        .into_iter()
        .map(|surface| MemoryPostureSurfaceParityRow {
            surface,
            shows_memory_classes: true,
            shows_saved_memory: true,
            shows_scoped_operations: true,
            reachable: true,
            qualification: MemoryPostureQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn downgrade_triggers() -> Vec<MemoryPostureDowngradeTrigger> {
    vec![
        MemoryPostureDowngradeTrigger::ProofStale,
        MemoryPostureDowngradeTrigger::PolicyBlocked,
        MemoryPostureDowngradeTrigger::ProviderUnavailable,
        MemoryPostureDowngradeTrigger::TrustNarrowing,
        MemoryPostureDowngradeTrigger::ScopeExpansionUnqualified,
        MemoryPostureDowngradeTrigger::UpstreamDependencyNarrowed,
        MemoryPostureDowngradeTrigger::DeleteFanOutIncomplete,
        MemoryPostureDowngradeTrigger::ExportFanOutIncomplete,
        MemoryPostureDowngradeTrigger::SavedMemoryWithoutAccountableOwner,
        MemoryPostureDowngradeTrigger::DeletionReceiptUnverified,
        MemoryPostureDowngradeTrigger::RetentionHoldUndisclosed,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MEMORY_DELETION_EXPORT_DOC_REF.to_owned(),
        MEMORY_DELETION_EXPORT_SCHEMA_REF.to_owned(),
        MEMORY_DELETION_EXPORT_DELETE_CONTRACT_REF.to_owned(),
        MEMORY_DELETION_EXPORT_RECONCILIATION_CONTRACT_REF.to_owned(),
        MEMORY_DELETION_EXPORT_MEMORY_OBJECT_SCHEMA_REF.to_owned(),
        MEMORY_DELETION_EXPORT_MEMORY_CLASSES_REF.to_owned(),
        MEMORY_DELETION_EXPORT_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        MEMORY_DELETION_EXPORT_M5_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> AiMemoryDeletionExportPosturePacketInput {
    AiMemoryDeletionExportPosturePacketInput {
        packet_id: PACKET_ID.to_owned(),
        posture_run_id: POSTURE_RUN_ID.to_owned(),
        display_label: "M5 AI memory deletion and export posture run".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        memory_class_coverage: memory_class_coverage(),
        explicit_saved_memory: explicit_saved_memory(),
        scoped_operations: scoped_operations(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: downgrade_triggers(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T11:46:00Z".to_owned(),
    }
}

fn packet() -> AiMemoryDeletionExportPosturePacket {
    AiMemoryDeletionExportPosturePacket::new(packet_input())
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = packet();
    assert_eq!(packet.record_kind, MEMORY_DELETION_EXPORT_RECORD_KIND);
    assert_eq!(packet.schema_version, MEMORY_DELETION_EXPORT_SCHEMA_VERSION);
    let json = packet.export_safe_json();
    assert!(json.contains("memory_class_coverage"));
}

#[test]
fn valid_packet_passes_validation() {
    assert!(packet().validate().is_empty());
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = packet();
    packet.schema_version = 99;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut packet = packet();
    packet.posture_run_id = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::MissingIdentity));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.pop();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::MissingSourceContracts));
}

#[test]
fn memory_class_coverage_empty_fails() {
    let mut packet = packet();
    packet.memory_class_coverage.class_rows.clear();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::MemoryClassCoverageEmpty));
}

#[test]
fn memory_class_coverage_missing_fails() {
    let mut packet = packet();
    packet
        .memory_class_coverage
        .class_rows
        .retain(|row| !matches!(row.state_class, AiStateClass::ExplicitSavedMemory));
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::MemoryClassCoverageMissing));
}

#[test]
fn hidden_memory_class_row_fails() {
    let mut packet = packet();
    packet.memory_class_coverage.class_rows[0].disclosed = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::HiddenMemoryClassRow));
}

#[test]
fn delete_fan_out_incomplete_fails() {
    let mut packet = packet();
    // A class with no hold that is not delete-covered is an incomplete fan-out.
    let row = packet
        .memory_class_coverage
        .class_rows
        .iter_mut()
        .find(|row| !row.retention_hold.holds_beyond_delete())
        .expect("non-held class present");
    row.delete_fan_out_covered = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::DeleteFanOutIncomplete));
}

#[test]
fn retention_hold_undisclosed_fails() {
    let mut packet = packet();
    let row = packet
        .memory_class_coverage
        .class_rows
        .iter_mut()
        .find(|row| row.retention_hold.holds_beyond_delete())
        .expect("held class present");
    row.hold_disclosed = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::RetentionHoldUndisclosed));
}

#[test]
fn export_fan_out_incomplete_fails() {
    let mut packet = packet();
    packet.memory_class_coverage.class_rows[0].export_fan_out_covered = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::ExportFanOutIncomplete));
}

#[test]
fn saved_memory_without_accountable_owner_fails() {
    let mut packet = packet();
    packet.explicit_saved_memory.saved_rows[0].actor_class = SavedMemoryActorClass::Unattributed;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::SavedMemoryWithoutAccountableOwner));
}

#[test]
fn saved_memory_not_consented_fails() {
    let mut packet = packet();
    packet.explicit_saved_memory.saved_rows[0].consent = SavedMemoryConsentClass::NotConsented;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::SavedMemoryNotConsented));
}

#[test]
fn saved_memory_not_revocable_fails() {
    let mut packet = packet();
    packet.explicit_saved_memory.saved_rows[0].revocable = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::SavedMemoryNotRevocable));
}

#[test]
fn hidden_saved_memory_row_fails() {
    let mut packet = packet();
    packet.explicit_saved_memory.saved_rows[0].disclosed = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::HiddenSavedMemoryRow));
}

#[test]
fn scoped_operation_set_empty_fails() {
    let mut packet = packet();
    packet.scoped_operations.operation_rows.clear();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::ScopedOperationSetEmpty));
}

#[test]
fn hidden_scoped_operation_fails() {
    let mut packet = packet();
    packet.scoped_operations.operation_rows[0].disclosed = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::HiddenScopedOperation));
}

#[test]
fn scoped_operation_missing_receipt_fails() {
    let mut packet = packet();
    packet.scoped_operations.operation_rows[0].receipt_ref = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::ScopedOperationMissingReceipt));
}

#[test]
fn scoped_operation_completeness_overstated_fails() {
    let mut packet = packet();
    let row = packet
        .scoped_operations
        .operation_rows
        .iter_mut()
        .find(|row| row.fan_out_completeness.claims_complete())
        .expect("complete operation present");
    row.all_classes_addressed = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::ScopedOperationCompletenessOverstated));
}

#[test]
fn deletion_receipt_unverified_fails() {
    let mut packet = packet();
    let row = packet
        .scoped_operations
        .operation_rows
        .iter_mut()
        .find(|row| row.fan_out_completeness.claims_complete())
        .expect("complete operation present");
    row.receipt_verification = ReceiptVerificationClass::PendingVerification;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::DeletionReceiptUnverified));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut packet = packet();
    packet.consumer_surface_parity.pop();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::ConsumerSurfaceCoverageMissing));
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut packet = packet();
    packet.consumer_surface_parity[0].reachable = false;
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut packet = packet();
    packet.explicit_saved_memory.saved_rows[0].entry_id = "/users/alice/secret".to_owned();
    assert!(packet
        .validate()
        .contains(&AiMemoryDeletionExportViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_renders() {
    let packet = packet();
    let md = packet.render_markdown_summary();
    assert!(md.contains("Memory classes"));
    assert!(md.contains("Saved memory"));
    assert!(md.contains("Scoped operations"));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_ai_memory_deletion_export_posture_export();
    assert!(
        result.is_ok(),
        "checked-in export must validate: {result:?}"
    );
}
