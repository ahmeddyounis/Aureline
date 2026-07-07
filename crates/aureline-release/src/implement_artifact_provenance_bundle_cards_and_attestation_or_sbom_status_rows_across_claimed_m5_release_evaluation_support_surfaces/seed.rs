//! Canonical seed builders for the M5 artifact-provenance-bundle-card /
//! attestation-or-SBOM status-row primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so
//! the in-code matrix, the artifact, the worked resolutions, and the fixtures never
//! drift.

use super::*;

/// Stable packet id for the canonical provenance-bundle-primitive packet.
pub const M5_PROVENANCE_BUNDLE_PRIMITIVE_PACKET_ID: &str =
    "m5-artifact-provenance-bundle-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full provenance state.
#[allow(clippy::too_many_arguments)]
fn case(
    artifact_identity_repr: &str,
    digest_set: &[&str],
    signature_status: M5SignatureStatus,
    attestation_status: M5AttestationStatus,
    sbom_status: M5SbomStatus,
    notice_bundle_status: M5SbomStatus,
    digest_lineage_state: M5DigestLineageState,
    inventory_format: M5InventoryFormat,
    inventory_scope: M5InventoryScope,
    inventory_freshness: M5InventoryFreshness,
    generator_version_repr: &str,
    inventory_export: M5InventoryExportAvailability,
    mirror_refs: &[&str],
    compare_available: bool,
    export_available: bool,
) -> M5ProvenanceBundleResolutionCase {
    M5ProvenanceBundleResolutionCase::resolved(M5ProvenanceBundleInput {
        artifact_identity_repr: artifact_identity_repr.to_owned(),
        digest_set: digest_set.iter().map(|s| (*s).to_owned()).collect(),
        signature_status,
        attestation_status,
        sbom_status,
        notice_bundle_status,
        digest_lineage_state,
        inventory_format,
        inventory_scope,
        inventory_freshness,
        generator_version_repr: generator_version_repr.to_owned(),
        inventory_export,
        mirror_refs: mirror_refs.iter().map(|s| (*s).to_owned()).collect(),
        compare_available,
        export_available,
    })
}

/// A base row with the shared fields filled in and the full anatomy, signature,
/// attestation, SBOM, digest-lineage, inventory-kind, inventory-format, scope,
/// freshness, export-availability, trust-posture, block-reason, next-action,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5ProvenanceBundleConsumerSurface,
    qualification: M5ReleaseCenterQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5ProvenanceBundleResolutionCase>,
) -> M5ProvenanceBundleRow {
    M5ProvenanceBundleRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5PublicationSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ProvenanceBundleAnatomyPart::ALL.to_vec(),
        signature_statuses: M5SignatureStatus::ALL.to_vec(),
        attestation_statuses: M5AttestationStatus::ALL.to_vec(),
        sbom_statuses: M5SbomStatus::ALL.to_vec(),
        digest_lineage_states: M5DigestLineageState::ALL.to_vec(),
        inventory_kinds: M5InventoryKind::ALL.to_vec(),
        inventory_formats: M5InventoryFormat::ALL.to_vec(),
        inventory_scopes: M5InventoryScope::ALL.to_vec(),
        inventory_freshnesses: M5InventoryFreshness::ALL.to_vec(),
        inventory_export_availabilities: M5InventoryExportAvailability::ALL.to_vec(),
        trust_postures: M5ProvenanceTrustPosture::ALL.to_vec(),
        block_reasons: M5ProvenanceBlockReason::ALL.to_vec(),
        next_actions: M5ProvenanceNextAction::ALL.to_vec(),
        export_fields: M5ProvenanceExportField::ALL.to_vec(),
        accessibility_routes: M5ReleaseCenterAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ReleaseCenterConsumerSurface::ReleaseCenterUi,
            M5ReleaseCenterConsumerSurface::HelpAbout,
            M5ReleaseCenterConsumerSurface::AdminConsole,
            M5ReleaseCenterConsumerSurface::EvaluationPack,
            M5ReleaseCenterConsumerSurface::MirrorConsole,
            M5ReleaseCenterConsumerSurface::SupportExport,
            M5ReleaseCenterConsumerSurface::CliInspect,
            M5ReleaseCenterConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5ReleaseCenterDowngradeTrigger::SignatureOrAttestationOverclaimed,
            M5ReleaseCenterDowngradeTrigger::SbomCompletenessOverstated,
            M5ReleaseCenterDowngradeTrigger::DigestLineageBrokenHidden,
            M5ReleaseCenterDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROVENANCE_BUNDLE_SCHEMA_REF,
            M5_PROVENANCE_BUNDLE_OBJECT_MODEL_REF,
            M5_PROVENANCE_BUNDLE_VERIFICATION_CONTRACT_REF,
        ]),
        example_resolutions,
        infers_trust_from_inventory_presence: false,
        conflates_signed_and_unsigned_provenance: false,
        overstates_sbom_completeness: false,
        drops_binding_on_compare_or_export: false,
    }
}

fn provenance_rows() -> Vec<M5ProvenanceBundleRow> {
    use M5AttestationStatus as Att;
    use M5DigestLineageState as Digest;
    use M5InventoryExportAvailability as Export;
    use M5InventoryFormat as Format;
    use M5InventoryFreshness as Fresh;
    use M5InventoryScope as Scope;
    use M5SbomStatus as Sbom;
    use M5SignatureStatus as Sig;

    let mut rows = Vec::new();

    // 1. Release-center provenance card — a fully proven artifact (signed + verified,
    //    attestation verified, digest pinned, full SBOM) with an intact compare/export
    //    binding, and a signature-broken artifact that is blocked with a self-contained
    //    banner (the proven / blocked and binding-intact coverage proof).
    rows.push(base_row(
        M5ProvenanceBundleConsumerSurface::ReleaseCenterProvenanceCard,
        M5ReleaseCenterQualificationClass::Stable,
        "Release-center provenance-card owner",
        "The release-center provenance card renders the shared provenance-bundle primitive so a signed-and-verified artifact with a verified attestation, a pinned immutable digest, and a full SBOM reads as trust-proven-exact with an intact compare/export binding, while an artifact whose signature is present but does not verify reads as blocked-signature-broken with a self-contained banner naming the reason, the bound digest, its mirror refs, and the re-sign-and-verify next action",
        "evidence:m5-provenance-center:001",
        vec![
            case(
                "artifact:aureline-core-runtime 5.2.0",
                &["sha256:aa11core", "sha512:bb22core"],
                Sig::SignedVerified,
                Att::AttestedVerified,
                Sbom::SbomComplete,
                Sbom::SbomComplete,
                Digest::ImmutableDigestPinned,
                Format::SpdxSbom,
                Scope::FullClosure,
                Fresh::InventoryFresh,
                "syft-1.18.0",
                Export::ExportAvailableOffline,
                &["mirror:us-east/aureline", "mirror:eu-west/aureline"],
                true,
                true,
            ),
            case(
                "artifact:aureline-shell 5.2.0",
                &["sha256:cc33shell"],
                Sig::SignatureBroken,
                Att::NoAttestation,
                Sbom::SbomMissing,
                Sbom::SbomMissing,
                Digest::ImmutableDigestPinned,
                Format::NotProvidedFormat,
                Scope::NotProvidedScope,
                Fresh::InventoryNotProvided,
                "",
                Export::ExportUnavailable,
                &["mirror:us-east/aureline"],
                true,
                true,
            ),
        ],
    ));

    // 2. Enterprise-evaluation provenance sheet — an artifact carrying a verified
    //    attestation and a complete SBOM but signed with an unverified key (inventory
    //    presence must NOT rescue it — it stays narrowed), and an artifact whose
    //    digest lineage is broken and is therefore blocked.
    rows.push(base_row(
        M5ProvenanceBundleConsumerSurface::EvaluationProvenanceSheet,
        M5ReleaseCenterQualificationClass::Stable,
        "Enterprise-evaluation provenance-sheet owner",
        "The enterprise-evaluation provenance sheet renders the shared primitive so an artifact carrying a verified attestation and a complete SBOM whose signing key is not yet verified reads as narrowed-signature-unverified — the SBOM and attestation presence never elevate it to proven — while an artifact whose immutable-digest lineage is broken reads as blocked-digest-lineage-broken with a rebuild-and-reconcile next action",
        "evidence:m5-provenance-evaluation:001",
        vec![
            case(
                "artifact:aureline-graph 5.2.0",
                &["sha256:dd44graph"],
                Sig::SignedUnverifiedKey,
                Att::AttestedVerified,
                Sbom::SbomComplete,
                Sbom::SbomComplete,
                Digest::DigestLineageContinuous,
                Format::CycloneDxSbom,
                Scope::DirectDependenciesOnly,
                Fresh::InventoryAging,
                "cyclonedx-gomod-1.4.0",
                Export::ExportAvailableOnlineOnly,
                &["mirror:us-east/aureline"],
                true,
                true,
            ),
            case(
                "artifact:aureline-registry 5.2.0",
                &["sha256:ee55registry"],
                Sig::SignedVerified,
                Att::NoAttestation,
                Sbom::SbomComplete,
                Sbom::SbomComplete,
                Digest::DigestLineageBroken,
                Format::SlsaProvenance,
                Scope::RuntimeClosureOnly,
                Fresh::InventoryFresh,
                "slsa-generator-2.0.0",
                Export::ExportOnRequest,
                &["mirror:eu-west/aureline"],
                true,
                true,
            ),
        ],
    ));

    // 3. CLI provenance inspect — a signed-and-verified artifact with an intact
    //    rebuild-matched digest but no attestation (trustworthy and honest that it is
    //    not attested), and an artifact whose signature, attestation, SBOM, and digest
    //    are all still being evaluated (blocked-unknown).
    rows.push(base_row(
        M5ProvenanceBundleConsumerSurface::CliProvenanceInspect,
        M5ReleaseCenterQualificationClass::Stable,
        "CLI provenance-inspect owner",
        "The CLI provenance-inspect surface renders the shared primitive so a signed-and-verified artifact whose clean-room rebuild reproduced the digest but which carries no attestation reads as trust-signed-not-attested — honest that it is not attested rather than overclaiming — while an artifact whose signature, attestation, SBOM, and digest are still being evaluated reads as blocked-provenance-unknown with a run-provenance-verification next action",
        "evidence:m5-provenance-cli:001",
        vec![
            case(
                "artifact:aureline-cli 5.2.0",
                &["sha256:ff66cli"],
                Sig::SignedVerified,
                Att::NoAttestation,
                Sbom::SbomComplete,
                Sbom::SbomComplete,
                Digest::RebuildDigestMatched,
                Format::SpdxSbom,
                Scope::FullClosure,
                Fresh::InventoryFresh,
                "syft-1.18.0",
                Export::ExportAvailableOffline,
                &["mirror:us-east/aureline"],
                true,
                true,
            ),
            case(
                "artifact:aureline-preview 5.3.0",
                &["sha256:aa77preview"],
                Sig::SignaturePending,
                Att::AttestationPending,
                Sbom::SbomGenerating,
                Sbom::SbomGenerating,
                Digest::DigestUnverified,
                Format::CycloneDxSbom,
                Scope::FullClosure,
                Fresh::InventoryRegenerating,
                "cyclonedx-gomod-1.4.0",
                Export::ExportAvailableOnlineOnly,
                &["mirror:us-east/aureline"],
                true,
                true,
            ),
        ],
    ));

    // 4. Admin provenance report — a signed-and-verified artifact whose attestation is
    //    present but not verified (narrowed, not proven), and an unsigned artifact with
    //    a complete SBOM that stays narrowed-signature-unverified (unsigned is never
    //    conflated with signed).
    rows.push(base_row(
        M5ProvenanceBundleConsumerSurface::AdminProvenanceReport,
        M5ReleaseCenterQualificationClass::Stable,
        "Admin provenance-report owner",
        "The admin provenance report renders the shared primitive so a signed-and-verified artifact whose attestation is present but not yet verified reads as narrowed-attestation-unverified with a verify-attestation next action, while an unsigned artifact carrying a complete SBOM reads as narrowed-signature-unverified — unsigned provenance is never conflated with signed",
        "evidence:m5-provenance-admin:001",
        vec![
            case(
                "artifact:aureline-update 5.2.0",
                &["sha256:bb88update"],
                Sig::SignedVerified,
                Att::AttestedUnverified,
                Sbom::SbomComplete,
                Sbom::SbomComplete,
                Digest::ImmutableDigestPinned,
                Format::SpdxSbom,
                Scope::FullClosure,
                Fresh::InventoryFresh,
                "syft-1.18.0",
                Export::ExportAvailableOffline,
                &["mirror:us-east/aureline"],
                true,
                true,
            ),
            case(
                "artifact:aureline-mirror 5.2.0",
                &["sha256:cc99mirror"],
                Sig::Unsigned,
                Att::NoAttestation,
                Sbom::SbomComplete,
                Sbom::SbomComplete,
                Digest::ImmutableDigestPinned,
                Format::SlsaProvenance,
                Scope::RuntimeClosureOnly,
                Fresh::InventoryFresh,
                "slsa-generator-2.0.0",
                Export::ExportOnRequest,
                &["mirror:eu-west/aureline"],
                true,
                true,
            ),
        ],
    ));

    // 5. Support provenance export — a signed-and-verified artifact whose SBOM is only
    //    partial (narrowed-inventory-incomplete, Partial state preserved), and a
    //    signed-and-verified artifact whose attestation has expired while its SBOM is
    //    stale (narrowed-attestation-unverified) — the same provenance vocabulary a
    //    support reviewer reads elsewhere.
    rows.push(base_row(
        M5ProvenanceBundleConsumerSurface::SupportProvenanceExport,
        M5ReleaseCenterQualificationClass::Stable,
        "Support provenance-export owner",
        "The support provenance export renders the shared primitive so a signed-and-verified artifact whose SBOM is only partial reads as narrowed-inventory-incomplete with the Partial state preserved and a complete-inventory next action, and a signed-and-verified artifact whose attestation has expired reads as narrowed-attestation-unverified — the same provenance and inventory vocabulary a support or evaluation reviewer reads across every surface",
        "evidence:m5-provenance-support:001",
        vec![
            case(
                "artifact:aureline-support-tools 5.2.0",
                &["sha256:dd00support"],
                Sig::SignedVerified,
                Att::NoAttestation,
                Sbom::SbomPartial,
                Sbom::SbomPartial,
                Digest::ImmutableDigestPinned,
                Format::SpdxSbom,
                Scope::PartialScope,
                Fresh::InventoryStale,
                "syft-1.18.0",
                Export::ExportAvailableOffline,
                &["mirror:us-east/aureline"],
                true,
                true,
            ),
            case(
                "artifact:aureline-docs 5.2.0",
                &["sha256:ee11docs"],
                Sig::SignedVerified,
                Att::AttestationExpired,
                Sbom::SbomStale,
                Sbom::SbomComplete,
                Digest::ImmutableDigestPinned,
                Format::CycloneDxSbom,
                Scope::DirectDependenciesOnly,
                Fresh::InventoryAging,
                "cyclonedx-gomod-1.4.0",
                Export::ExportAvailableOffline,
                &["mirror:eu-west/aureline"],
                true,
                true,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ProvenanceBundleGovernanceReview {
    M5ProvenanceBundleGovernanceReview {
        one_primitive_carries_provenance_truth: true,
        inspectable_without_unpacking_archives: true,
        trust_never_derived_from_inventory_presence: true,
        inventory_rows_separate_from_signature: true,
        not_provided_and_partial_preserved: true,
        compare_export_keeps_binding_intact: true,
        blocked_state_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_next_action: true,
        support_export_reconstructs_provenance_truth: true,
        no_surface_invents_second_provenance_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ProvenanceBundleConsumerProjection {
    M5ProvenanceBundleConsumerProjection {
        provenance_surfaces_consume_shared_primitive: true,
        trust_resolver_reads_single_source: true,
        inventory_rows_read_single_source: true,
        compare_export_binding_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ProvenanceBundleProofFreshness {
    M5ProvenanceBundleProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ProvenanceBundleReleasePosture {
    M5ProvenanceBundleReleasePosture {
        release_packet_ref: M5_PROVENANCE_BUNDLE_ARTIFACT_REF.to_owned(),
        provenance_audit_ref: M5_PROVENANCE_BUNDLE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROVENANCE_BUNDLE_SCHEMA_REF,
        M5_PROVENANCE_BUNDLE_DOC_REF,
        M5_PROVENANCE_BUNDLE_COMPONENT_MATRIX_REF,
        M5_PROVENANCE_BUNDLE_OBJECT_MODEL_REF,
        M5_PROVENANCE_BUNDLE_VERIFICATION_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 provenance-bundle-primitive packet.
pub fn seeded_m5_provenance_bundle_primitive_packet() -> M5ProvenanceBundlePrimitivePacket {
    M5ProvenanceBundlePrimitivePacket::new(M5ProvenanceBundlePrimitivePacketInput {
        packet_id: M5_PROVENANCE_BUNDLE_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 artifact-provenance-bundle card and attestation/SBOM status-row primitive: artifact identity, digest set, signature state, attestation state, SBOM/notice bundle state, digest lineage, inventory format/scope/freshness/export, mirror refs, and compare/export truth"
                .to_owned(),
        provenance_rows: provenance_rows(),
        vocabulary_set: M5ProvenanceBundleVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the enterprise-evaluation provenance sheet is held at Beta
/// because a slice of evaluation exports do not yet render the notice-bundle status
/// row on every profile; every consumer stays visible.
pub fn seeded_m5_provenance_bundle_primitive_evaluation_provenance_sheet_beta_narrowed(
) -> M5ProvenanceBundlePrimitivePacket {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.packet_id =
        "m5-artifact-provenance-bundle-card-primitive:evaluation-beta:0001".to_owned();
    let row = packet
        .provenance_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5ProvenanceBundleConsumerSurface::EvaluationProvenanceSheet
        })
        .expect("evaluation provenance sheet present");
    row.qualification = M5ReleaseCenterQualificationClass::Beta;
    packet
}

/// Narrowed variant: the CLI provenance-inspect surface is narrowed to Preview
/// pending self-contained-banner parity proof across every headless export path;
/// every consumer stays visible.
pub fn seeded_m5_provenance_bundle_primitive_cli_provenance_inspect_preview_narrowed(
) -> M5ProvenanceBundlePrimitivePacket {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.packet_id =
        "m5-artifact-provenance-bundle-card-primitive:cli-provenance-inspect-preview:0001"
            .to_owned();
    let row = packet
        .provenance_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ProvenanceBundleConsumerSurface::CliProvenanceInspect)
        .expect("cli provenance-inspect row present");
    row.qualification = M5ReleaseCenterQualificationClass::Preview;
    packet
}
