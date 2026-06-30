//! Canonical seed builders for the M5 post-install disclosure panels.
//!
//! These builders are the single producer of the checked-in panel set, the
//! per-family panel exports, and the narrowed fixtures. The headless emitter and
//! the inline tests both call them so the in-code panels, the artifacts, and the
//! fixtures never drift. Every panel mirrors a worked governance case under
//! `fixtures/governance/post_install_cases/` so the help lane and the governance
//! corpus speak one truth.

use super::*;

/// Stable packet id for the canonical post-install disclosure panel set.
pub const M5_POST_INSTALL_DISCLOSURE_PANEL_SET_ID: &str =
    "m5-post-install-disclosure-panels:stable:0001";

/// Mint timestamp pinned by the panel-set builder.
const SEED_MINTED_AT: &str = "2026-06-30T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn opt(value: &str) -> Option<String> {
    Some(value.to_owned())
}

fn access_point(
    access_point_class: AccessPointClass,
    reachability_class: ReachabilityClass,
    action_ref: Option<String>,
    disclosure: &str,
) -> AccessPoint {
    AccessPoint {
        access_point_class,
        reachability_class,
        action_ref,
        disclosure: disclosure.to_owned(),
    }
}

fn action(
    action_ref: &str,
    action_class: ActionClass,
    label: &str,
    target_ref: Option<String>,
    availability: ReachabilityClass,
) -> Action {
    Action {
        action_ref: action_ref.to_owned(),
        action_class,
        label: label.to_owned(),
        target_ref,
        availability,
    }
}

fn missing_row(
    data_class: DataClass,
    missing_state: MissingState,
    visible_label: &str,
    disclosure: &str,
    resolution_action_ref: Option<String>,
) -> MissingOrPartialDataRow {
    MissingOrPartialDataRow {
        data_class,
        missing_state,
        visible_label: visible_label.to_owned(),
        disclosure: disclosure.to_owned(),
        resolution_action_ref,
    }
}

/// Installed official, signed, stable desktop product build.
fn panel_desktop_official_signed_build() -> PostInstallDisclosureRecord {
    PostInstallDisclosureRecord {
        schema_version: POST_INSTALL_DISCLOSURE_SCHEMA_VERSION,
        record_kind: POST_INSTALL_DISCLOSURE_RECORD_KIND.to_owned(),
        disclosure_id: "post_install_disclosure:desktop.official.signed_stable".to_owned(),
        emitted_at: "2026-04-29T18:20:00Z".to_owned(),
        review_context: ReviewContext {
            surface_subject_kind: SurfaceSubjectKind::ProductBuild,
            surface_title: "Installed product build disclosure".to_owned(),
            boundary_statement:
                "This surface describes the installed desktop product build, not extensions, mirrored transport artifacts, or generated user exports."
                    .to_owned(),
            primary_artifact_ref: "artifact:aureline.desktop.macos.universal.2_1_0".to_owned(),
        },
        artifact: Artifact {
            artifact_class: ArtifactClass::DesktopBuild,
            display_name: "Aureline Desktop 2.1.0".to_owned(),
            artifact_identity_ref: "artifact:aureline.desktop.macos.universal.2_1_0".to_owned(),
            version_or_digest_ref: "digest:sha256.desktop.macos.universal.2_1_0".to_owned(),
            build_id: opt("build:release.stable.2_1_0"),
            channel: ChannelClass::Stable,
            exact_build_identity_ref: opt("exact_build:desktop.macos.universal.2_1_0"),
            installer_receipt_ref: opt("installer_receipt:macos.pkg.2_1_0"),
            generated_artifact_lineage_ref: None,
        },
        source: Source {
            source_class: SourceClass::Official,
            source_label: SourceLabel::Official,
            origin_ref: opt("producer:ci_release.desktop"),
            upstream_origin_ref: opt("origin:official.update_feed.stable"),
            mirror_ref: None,
            side_load_review_ref: None,
            acquired_via: AcquiredVia::OfficialUpdateFeed,
            source_disclosure:
                "Build was installed from the official stable update feed and resolves to one exact-build identity."
                    .to_owned(),
        },
        verification: Verification {
            signature_state: SignatureState::SignedVerified,
            attestation_state: AttestationState::AttestationVerified,
            checksum_state: ChecksumState::ChecksumVerified,
            revocation_state: RevocationState::RevocationCurrent,
            revocation_freshness_class: FreshnessClass::Current,
            revocation_snapshot_ref: opt("revocation_snapshot:stable.2_1_0.current"),
            checked_at: opt("2026-04-29T18:18:00Z"),
            verification_evidence_refs: strings(&[
                "signature:stable.2_1_0.desktop.macos",
                "attestation:slsa.2_1_0.desktop.macos",
                "checksum_bundle:stable.2_1_0",
                "revocation:none.2_1_0",
            ]),
            verification_disclosure:
                "Signature, attestation, checksum, and revocation checks are current for the installed product build."
                    .to_owned(),
        },
        notice_inventory: NoticeInventory {
            license_state: LicenseState::LicenseAllowedWithNotice,
            license_expression_ref: opt("license_expression:desktop.2_1_0"),
            notice_state: NoticeState::NoticeComplete,
            notice_inventory_state: NoticeInventoryState::InventoryAvailable,
            notice_inventory_refs: strings(&[
                "notice_inventory:desktop.2_1_0",
                "third_party_notice:desktop.2_1_0",
            ]),
            sbom_state: SbomState::SbomAttachedVerified,
            sbom_formats: vec![SbomFormat::SpdxJson, SbomFormat::CyclonedxJson],
            notice_disclosure:
                "Third-party notices and verified SPDX/CycloneDX SBOM refs are available after install."
                    .to_owned(),
        },
        visible_cues: VisibleCues {
            source_cue: "Official".to_owned(),
            provenance_cue:
                "Signature verified and attestation verified for build build:release.stable.2_1_0."
                    .to_owned(),
            license_cue: "License inventory is allowed with bundled notice obligations.".to_owned(),
            notice_cue:
                "Third-party notice inventory is available from About, diagnostics export, and review sheets."
                    .to_owned(),
            revocation_cue: "Revocation metadata is current for the installed stable build."
                .to_owned(),
            missing_or_partial_data: vec![],
        },
        access_points: vec![
            access_point(
                AccessPointClass::About,
                ReachabilityClass::Available,
                opt("action:about.open_notices.desktop.2_1_0"),
                "About opens notices, SBOM export, and provenance detail for the running product build.",
            ),
            access_point(
                AccessPointClass::UpdateCenter,
                ReachabilityClass::Available,
                opt("action:update_center.inspect_provenance.desktop.2_1_0"),
                "Update center shows the same build id, channel, revocation, and rollback-linked evidence refs.",
            ),
            access_point(
                AccessPointClass::InstalledStateInspector,
                ReachabilityClass::Available,
                opt("action:installed_state.inspect.desktop.2_1_0"),
                "Installed-state inspector identifies this as the product build subject.",
            ),
            access_point(
                AccessPointClass::DiagnosticsExport,
                ReachabilityClass::Available,
                opt("action:diagnostics.export_disclosure.desktop.2_1_0"),
                "Diagnostics export carries the disclosure id and evidence refs without raw SBOM or notice bodies.",
            ),
            access_point(
                AccessPointClass::ReviewSheet,
                ReachabilityClass::AvailableReadOnly,
                opt("action:review_sheet.open.desktop.2_1_0"),
                "Review sheets can reopen the installed build disclosure read-only after install.",
            ),
        ],
        actions: vec![
            action(
                "action:about.open_notices.desktop.2_1_0",
                ActionClass::OpenNotices,
                "Open notices",
                opt("notice_inventory:desktop.2_1_0"),
                ReachabilityClass::Available,
            ),
            action(
                "action:about.export_sbom.desktop.2_1_0",
                ActionClass::ExportSbom,
                "Export SBOM",
                opt("sbom:desktop.2_1_0"),
                ReachabilityClass::Available,
            ),
            action(
                "action:update_center.inspect_provenance.desktop.2_1_0",
                ActionClass::InspectProvenance,
                "Inspect provenance",
                opt("post_install_disclosure:desktop.official.signed_stable"),
                ReachabilityClass::Available,
            ),
            action(
                "action:diagnostics.export_disclosure.desktop.2_1_0",
                ActionClass::ExportDisclosurePacket,
                "Export disclosure",
                opt("diagnostics_export:desktop.2_1_0"),
                ReachabilityClass::Available,
            ),
        ],
        redistribution: Redistribution {
            redistribution_hint_class: RedistributionHintClass::NotApplicable,
            redistribution_disclosure:
                "This disclosure describes an installed product build; generated user-artifact redistribution rules do not apply."
                    .to_owned(),
            required_notice_refs: vec![],
            required_license_refs: vec![],
            required_lineage_refs: vec![],
            policy_refs: vec![],
        },
        linkage: Linkage {
            provenance_badge_refs: strings(&["provenance_badge:release.official.macos_arm64"]),
            update_manifest_refs: strings(&["update_manifest:stable.public.2_1_0"]),
            install_review_refs: strings(&["install_review:desktop.macos.2_1_0"]),
            extension_or_pack_review_refs: vec![],
            generated_lineage_refs: vec![],
            release_evidence_refs: strings(&["release_evidence:stable.2_1_0"]),
            support_bundle_refs: strings(&["support_bundle:stable.2_1_0.metadata"]),
            diagnostics_export_refs: strings(&["diagnostics_export:desktop.2_1_0"]),
            mirror_or_offline_receipt_refs: vec![],
        },
        export_projection: ExportProjection {
            diagnostics_export_refs: strings(&["diagnostics_export:desktop.2_1_0"]),
            support_bundle_refs: strings(&["support_bundle:stable.2_1_0.metadata"]),
            public_proof_refs: strings(&["public_proof:stable.2_1_0.supply_chain"]),
            offline_review_refs: vec![],
            redaction_class: RedactionClass::PublicProofSafe,
            omission_reasons: vec![],
        },
        narrative_refs: strings(&[
            "docs/governance/post_install_notice_and_provenance_contract.md",
            "docs/governance/provenance_badge_contract.md",
            "docs/release/release_artifact_graph.md",
        ]),
    }
}

/// Side-loaded extension package with missing license/notice/SBOM/attestation.
fn panel_side_loaded_extension() -> PostInstallDisclosureRecord {
    PostInstallDisclosureRecord {
        schema_version: POST_INSTALL_DISCLOSURE_SCHEMA_VERSION,
        record_kind: POST_INSTALL_DISCLOSURE_RECORD_KIND.to_owned(),
        disclosure_id: "post_install_disclosure:extension.side_loaded.local_archive".to_owned(),
        emitted_at: "2026-04-29T18:40:00Z".to_owned(),
        review_context: ReviewContext {
            surface_subject_kind: SurfaceSubjectKind::ExtensionPackage,
            surface_title: "Side-loaded extension disclosure".to_owned(),
            boundary_statement:
                "This surface describes one side-loaded extension package, not the official product build or its update channel."
                    .to_owned(),
            primary_artifact_ref: "artifact:extension.local.theme_lab.0_4_0".to_owned(),
        },
        artifact: Artifact {
            artifact_class: ArtifactClass::ExtensionPackage,
            display_name: "Theme Lab extension 0.4.0".to_owned(),
            artifact_identity_ref: "artifact:extension.local.theme_lab.0_4_0".to_owned(),
            version_or_digest_ref: "digest:sha256.extension.theme_lab.0_4_0".to_owned(),
            build_id: None,
            channel: ChannelClass::Local,
            exact_build_identity_ref: None,
            installer_receipt_ref: opt("installer_receipt:extension.side_load.theme_lab.0_4_0"),
            generated_artifact_lineage_ref: None,
        },
        source: Source {
            source_class: SourceClass::SideLoaded,
            source_label: SourceLabel::SideLoaded,
            origin_ref: None,
            upstream_origin_ref: None,
            mirror_ref: None,
            side_load_review_ref: opt("side_load_review:extension.theme_lab.0_4_0"),
            acquired_via: AcquiredVia::LocalFilePicker,
            source_disclosure:
                "Extension archive was selected locally and admitted through a side-load review, not through the registry or update service."
                    .to_owned(),
        },
        verification: Verification {
            signature_state: SignatureState::SignedUnverified,
            attestation_state: AttestationState::AttestationMissing,
            checksum_state: ChecksumState::ChecksumVerified,
            revocation_state: RevocationState::RevocationUnknown,
            revocation_freshness_class: FreshnessClass::Unknown,
            revocation_snapshot_ref: None,
            checked_at: opt("2026-04-29T18:37:00Z"),
            verification_evidence_refs: strings(&[
                "checksum:extension.theme_lab.0_4_0",
                "side_load_review:extension.theme_lab.0_4_0",
            ]),
            verification_disclosure:
                "Local checksum was recorded, but signature, attestation, and revocation freshness are not verified through a registry."
                    .to_owned(),
        },
        notice_inventory: NoticeInventory {
            license_state: LicenseState::LicenseUnknown,
            license_expression_ref: None,
            notice_state: NoticeState::NoticeUnknown,
            notice_inventory_state: NoticeInventoryState::InventoryMissing,
            notice_inventory_refs: vec![],
            sbom_state: SbomState::SbomMissing,
            sbom_formats: vec![],
            notice_disclosure: "The side-loaded archive did not provide license, notice, or SBOM inventory data."
                .to_owned(),
        },
        visible_cues: VisibleCues {
            source_cue: "Side-loaded".to_owned(),
            provenance_cue:
                "Side-load review recorded a checksum, but registry provenance and attestation are not provided."
                    .to_owned(),
            license_cue: "License state is unknown for this extension.".to_owned(),
            notice_cue: "Notice inventory is missing and remains visible in installed state.".to_owned(),
            revocation_cue: "No current revocation feed applies to this side-loaded extension.".to_owned(),
            missing_or_partial_data: vec![
                missing_row(
                    DataClass::Attestation,
                    MissingState::NotProvided,
                    "Attestation not provided",
                    "The archive did not include a provenance attestation.",
                    opt("action:extension_details.verify_now.theme_lab"),
                ),
                missing_row(
                    DataClass::Sbom,
                    MissingState::NotProvided,
                    "SBOM not provided",
                    "The archive did not include an SPDX or CycloneDX SBOM.",
                    None,
                ),
                missing_row(
                    DataClass::License,
                    MissingState::Unknown,
                    "License unknown",
                    "No license expression was resolved from the archive.",
                    opt("action:review_sheet.open.extension.theme_lab"),
                ),
                missing_row(
                    DataClass::NoticeInventory,
                    MissingState::NotProvided,
                    "Notices not provided",
                    "No third-party notice inventory was resolved for this side-loaded extension.",
                    opt("action:review_sheet.open.extension.theme_lab"),
                ),
                missing_row(
                    DataClass::RevocationSnapshot,
                    MissingState::Unknown,
                    "Revocation unknown",
                    "The extension is outside the registry revocation feed.",
                    opt("action:extension_details.verify_now.theme_lab"),
                ),
            ],
        },
        access_points: vec![
            access_point(
                AccessPointClass::About,
                ReachabilityClass::Available,
                opt("action:about.open_extension_disclosure.theme_lab"),
                "About links to installed extension disclosures separately from product build notices.",
            ),
            access_point(
                AccessPointClass::UpdateCenter,
                ReachabilityClass::AvailableReadOnly,
                opt("action:update_center.inspect_side_loaded_extension.theme_lab"),
                "Update center shows the extension as side-loaded and not registry-updated.",
            ),
            access_point(
                AccessPointClass::InstalledStateInspector,
                ReachabilityClass::Available,
                opt("action:installed_state.inspect.extension.theme_lab"),
                "Installed-state inspector identifies this as an extension package.",
            ),
            access_point(
                AccessPointClass::DiagnosticsExport,
                ReachabilityClass::Available,
                opt("action:diagnostics.export_disclosure.extension.theme_lab"),
                "Diagnostics export includes the side-load review ref and missing-data rows.",
            ),
            access_point(
                AccessPointClass::ReviewSheet,
                ReachabilityClass::Available,
                opt("action:review_sheet.open.extension.theme_lab"),
                "Review sheet reopens the side-load decision and missing license/notice state.",
            ),
            access_point(
                AccessPointClass::ExtensionDetails,
                ReachabilityClass::Available,
                opt("action:extension_details.open.theme_lab"),
                "Extension details exposes checksum, support limitation, and missing evidence rows.",
            ),
            access_point(
                AccessPointClass::MarketplaceOrPackageDetail,
                ReachabilityClass::AvailableReadOnly,
                opt("action:marketplace.inspect_side_loaded.theme_lab"),
                "Marketplace / package detail marks the package side-loaded and links the side-load review rather than a registry listing.",
            ),
        ],
        actions: vec![
            action(
                "action:extension_details.open.theme_lab",
                ActionClass::OpenExtensionDetails,
                "Open extension details",
                opt("artifact:extension.local.theme_lab.0_4_0"),
                ReachabilityClass::Available,
            ),
            action(
                "action:extension_details.verify_now.theme_lab",
                ActionClass::VerifyNow,
                "Verify now",
                opt("side_load_review:extension.theme_lab.0_4_0"),
                ReachabilityClass::Available,
            ),
            action(
                "action:review_sheet.open.extension.theme_lab",
                ActionClass::OpenReviewSheet,
                "Open side-load review",
                opt("side_load_review:extension.theme_lab.0_4_0"),
                ReachabilityClass::Available,
            ),
            action(
                "action:diagnostics.export_disclosure.extension.theme_lab",
                ActionClass::ExportDisclosurePacket,
                "Export disclosure",
                opt("diagnostics_export:extension.theme_lab"),
                ReachabilityClass::Available,
            ),
        ],
        redistribution: Redistribution {
            redistribution_hint_class: RedistributionHintClass::UnknownReviewRequired,
            redistribution_disclosure:
                "Redistribution is not assessed because license and notice state are unknown.".to_owned(),
            required_notice_refs: vec![],
            required_license_refs: vec![],
            required_lineage_refs: vec![],
            policy_refs: strings(&["policy:side_loaded_extension.redistribution_review"]),
        },
        linkage: Linkage {
            provenance_badge_refs: strings(&["provenance_badge:archive.side_loaded.theme_lab"]),
            update_manifest_refs: vec![],
            install_review_refs: strings(&["side_load_review:extension.theme_lab.0_4_0"]),
            extension_or_pack_review_refs: strings(&["extension_review:theme_lab.0_4_0"]),
            generated_lineage_refs: vec![],
            release_evidence_refs: vec![],
            support_bundle_refs: strings(&["support_bundle:extension.theme_lab.metadata"]),
            diagnostics_export_refs: strings(&["diagnostics_export:extension.theme_lab"]),
            mirror_or_offline_receipt_refs: vec![],
        },
        export_projection: ExportProjection {
            diagnostics_export_refs: strings(&["diagnostics_export:extension.theme_lab"]),
            support_bundle_refs: strings(&["support_bundle:extension.theme_lab.metadata"]),
            public_proof_refs: vec![],
            offline_review_refs: vec![],
            redaction_class: RedactionClass::SupportRedacted,
            omission_reasons: strings(&[
                "Raw local archive path and raw extension bytes are omitted; checksum and side-load review refs remain.",
            ]),
        },
        narrative_refs: strings(&[
            "docs/governance/post_install_notice_and_provenance_contract.md",
            "docs/governance/provenance_badge_contract.md",
        ]),
    }
}

/// Mirrored offline update bundle whose revocation snapshot is stale.
fn panel_mirrored_offline_artifact() -> PostInstallDisclosureRecord {
    PostInstallDisclosureRecord {
        schema_version: POST_INSTALL_DISCLOSURE_SCHEMA_VERSION,
        record_kind: POST_INSTALL_DISCLOSURE_RECORD_KIND.to_owned(),
        disclosure_id: "post_install_disclosure:mirror.offline_bundle.stale_revocation".to_owned(),
        emitted_at: "2026-04-29T18:30:00Z".to_owned(),
        review_context: ReviewContext {
            surface_subject_kind: SurfaceSubjectKind::MirroredTransportArtifact,
            surface_title: "Mirrored offline update bundle disclosure".to_owned(),
            boundary_statement:
                "This surface describes the mirrored transport bundle and its upstream origin, not the locally running product build alone."
                    .to_owned(),
            primary_artifact_ref: "artifact:offline_update_bundle.stable.2_1_0".to_owned(),
        },
        artifact: Artifact {
            artifact_class: ArtifactClass::OfflineBundle,
            display_name: "Offline stable update bundle 2.1.0".to_owned(),
            artifact_identity_ref: "artifact:offline_update_bundle.stable.2_1_0".to_owned(),
            version_or_digest_ref: "digest:sha256.offline_update_bundle.stable.2_1_0".to_owned(),
            build_id: opt("build:release.stable.2_1_0"),
            channel: ChannelClass::Stable,
            exact_build_identity_ref: opt("exact_build:update_bundle.stable.2_1_0"),
            installer_receipt_ref: opt("installer_receipt:offline_bundle.enterprise.2026_04_29"),
            generated_artifact_lineage_ref: None,
        },
        source: Source {
            source_class: SourceClass::Mirrored,
            source_label: SourceLabel::Mirrored,
            origin_ref: opt("producer:ci_release.update_bundle"),
            upstream_origin_ref: opt("origin:official.update_bundle.stable.2_1_0"),
            mirror_ref: opt("mirror_snapshot:enterprise.offline.2026_04_27"),
            side_load_review_ref: None,
            acquired_via: AcquiredVia::OfflineBundle,
            source_disclosure:
                "Bundle came through an enterprise offline mirror that preserves official origin identity and adds a mirror receipt."
                    .to_owned(),
        },
        verification: Verification {
            signature_state: SignatureState::SignedVerified,
            attestation_state: AttestationState::AttestationVerified,
            checksum_state: ChecksumState::ChecksumVerified,
            revocation_state: RevocationState::RevocationSnapshotStale,
            revocation_freshness_class: FreshnessClass::StaleRequiresReview,
            revocation_snapshot_ref: opt("revocation_snapshot:enterprise.offline.2026_04_27"),
            checked_at: opt("2026-04-29T18:28:00Z"),
            verification_evidence_refs: strings(&[
                "signature:update_bundle.stable.2_1_0",
                "attestation:slsa.update_bundle.stable.2_1_0",
                "checksum_bundle:update_bundle.stable.2_1_0",
                "revocation_snapshot:enterprise.offline.2026_04_27",
                "manual_import_receipt:enterprise.offline.2026_04_29",
            ]),
            verification_disclosure:
                "Origin signature and attestation verify, but revocation data is stale relative to the configured freshness floor."
                    .to_owned(),
        },
        notice_inventory: NoticeInventory {
            license_state: LicenseState::LicenseAllowedWithNotice,
            license_expression_ref: opt("license_expression:update_bundle.stable.2_1_0"),
            notice_state: NoticeState::NoticeComplete,
            notice_inventory_state: NoticeInventoryState::InventoryAvailable,
            notice_inventory_refs: strings(&["notice_inventory:update_bundle.stable.2_1_0"]),
            sbom_state: SbomState::SbomAttachedVerified,
            sbom_formats: vec![SbomFormat::SpdxJson, SbomFormat::CyclonedxJson],
            notice_disclosure:
                "Notice inventory and SBOM refs are preserved by the mirror and exportable in the offline review packet."
                    .to_owned(),
        },
        visible_cues: VisibleCues {
            source_cue: "Mirrored".to_owned(),
            provenance_cue: "Official origin verifies through the mirror receipt.".to_owned(),
            license_cue: "License inventory is allowed with bundled notices.".to_owned(),
            notice_cue: "Notices are available from the offline bundle.".to_owned(),
            revocation_cue:
                "Revocation snapshot is stale and must be refreshed before making current-security claims."
                    .to_owned(),
            missing_or_partial_data: vec![missing_row(
                DataClass::RevocationSnapshot,
                MissingState::Stale,
                "Stale revocation snapshot",
                "The latest available mirror revocation snapshot is older than the freshness floor.",
                opt("action:update_center.refresh_revocation_snapshot.offline_bundle"),
            )],
        },
        access_points: vec![
            access_point(
                AccessPointClass::About,
                ReachabilityClass::AvailableReadOnly,
                opt("action:about.open_mirror_disclosure.offline_bundle"),
                "About links from the running build to the mirrored bundle disclosure used for install.",
            ),
            access_point(
                AccessPointClass::UpdateCenter,
                ReachabilityClass::Available,
                opt("action:update_center.refresh_revocation_snapshot.offline_bundle"),
                "Update center exposes stale revocation freshness and the refresh/manual-import path.",
            ),
            access_point(
                AccessPointClass::InstalledStateInspector,
                ReachabilityClass::Available,
                opt("action:installed_state.inspect.offline_bundle"),
                "Installed-state inspector identifies the subject as a mirrored transport artifact.",
            ),
            access_point(
                AccessPointClass::DiagnosticsExport,
                ReachabilityClass::Available,
                opt("action:diagnostics.export_disclosure.offline_bundle"),
                "Diagnostics export preserves mirror receipt and stale revocation refs.",
            ),
            access_point(
                AccessPointClass::ReviewSheet,
                ReachabilityClass::Available,
                opt("action:review_sheet.open.offline_bundle"),
                "Review sheet keeps origin verification and mirror freshness separate.",
            ),
            access_point(
                AccessPointClass::OfflineReview,
                ReachabilityClass::Available,
                opt("action:offline_review.open_bundle_receipt"),
                "Offline review can inspect the manual import receipt and mirror snapshot.",
            ),
        ],
        actions: vec![
            action(
                "action:update_center.refresh_revocation_snapshot.offline_bundle",
                ActionClass::RefreshRevocationSnapshot,
                "Refresh revocation snapshot",
                opt("revocation_snapshot:enterprise.offline.latest"),
                ReachabilityClass::Available,
            ),
            action(
                "action:review_sheet.open.offline_bundle",
                ActionClass::OpenReviewSheet,
                "Open review sheet",
                opt("review_sheet:offline_bundle.stable.2_1_0"),
                ReachabilityClass::Available,
            ),
            action(
                "action:offline_review.open_bundle_receipt",
                ActionClass::InspectProvenance,
                "Inspect mirror receipt",
                opt("manual_import_receipt:enterprise.offline.2026_04_29"),
                ReachabilityClass::Available,
            ),
        ],
        redistribution: Redistribution {
            redistribution_hint_class: RedistributionHintClass::NotApplicable,
            redistribution_disclosure:
                "This disclosure describes a mirrored transport artifact; generated user-artifact redistribution rules do not apply."
                    .to_owned(),
            required_notice_refs: vec![],
            required_license_refs: vec![],
            required_lineage_refs: vec![],
            policy_refs: strings(&["policy:offline_mirror.freshness_floor"]),
        },
        linkage: Linkage {
            provenance_badge_refs: strings(&["provenance_badge:mirror.official.offline_bundle"]),
            update_manifest_refs: strings(&["update_manifest:stable.public.2_1_0"]),
            install_review_refs: strings(&["install_review:offline_bundle.enterprise.2_1_0"]),
            extension_or_pack_review_refs: vec![],
            generated_lineage_refs: vec![],
            release_evidence_refs: strings(&["release_evidence:stable.2_1_0"]),
            support_bundle_refs: strings(&["support_bundle:offline_bundle.stable.2_1_0.metadata"]),
            diagnostics_export_refs: strings(&["diagnostics_export:offline_bundle.2_1_0"]),
            mirror_or_offline_receipt_refs: strings(&[
                "manual_import_receipt:enterprise.offline.2026_04_29",
                "mirror_snapshot:enterprise.offline.2026_04_27",
            ]),
        },
        export_projection: ExportProjection {
            diagnostics_export_refs: strings(&["diagnostics_export:offline_bundle.2_1_0"]),
            support_bundle_refs: strings(&["support_bundle:offline_bundle.stable.2_1_0.metadata"]),
            public_proof_refs: vec![],
            offline_review_refs: strings(&["offline_review:bundle.stable.2_1_0"]),
            redaction_class: RedactionClass::SupportRedacted,
            omission_reasons: strings(&[
                "Public proof omits private mirror receipt detail while preserving stale revocation state.",
            ]),
        },
        narrative_refs: strings(&[
            "docs/governance/post_install_notice_and_provenance_contract.md",
            "docs/release/update_and_rollback_contract.md",
            "docs/governance/provenance_badge_contract.md",
        ]),
    }
}

/// Generated export from an official build, carrying a redistribution hint.
fn panel_generated_export() -> PostInstallDisclosureRecord {
    PostInstallDisclosureRecord {
        schema_version: POST_INSTALL_DISCLOSURE_SCHEMA_VERSION,
        record_kind: POST_INSTALL_DISCLOSURE_RECORD_KIND.to_owned(),
        disclosure_id:
            "post_install_disclosure:generated_export.support_report.redistribution_review"
                .to_owned(),
        emitted_at: "2026-04-29T18:50:00Z".to_owned(),
        review_context: ReviewContext {
            surface_subject_kind: SurfaceSubjectKind::GeneratedUserArtifact,
            surface_title: "Generated export disclosure".to_owned(),
            boundary_statement:
                "This surface describes a generated user artifact produced by Aureline, not an official product release artifact."
                    .to_owned(),
            primary_artifact_ref: "artifact:export.support_report.team_readiness.2026_04_29"
                .to_owned(),
        },
        artifact: Artifact {
            artifact_class: ArtifactClass::GeneratedExport,
            display_name: "Team readiness support report export".to_owned(),
            artifact_identity_ref: "artifact:export.support_report.team_readiness.2026_04_29"
                .to_owned(),
            version_or_digest_ref: "digest:sha256.export.support_report.team_readiness.2026_04_29"
                .to_owned(),
            build_id: opt("build:release.stable.2_1_0"),
            channel: ChannelClass::Stable,
            exact_build_identity_ref: opt("exact_build:desktop.macos.universal.2_1_0"),
            installer_receipt_ref: None,
            generated_artifact_lineage_ref: opt(
                "generated_lineage:export.support_report.team_readiness.2026_04_29",
            ),
        },
        source: Source {
            source_class: SourceClass::Official,
            source_label: SourceLabel::Official,
            origin_ref: opt("producer:aureline.exporter.support_report"),
            upstream_origin_ref: opt("origin:installed_product_build.stable.2_1_0"),
            mirror_ref: None,
            side_load_review_ref: None,
            acquired_via: AcquiredVia::GeneratedExportFlow,
            source_disclosure:
                "Export was generated by an official installed build, but the artifact itself is user-generated and scoped by lineage refs."
                    .to_owned(),
        },
        verification: Verification {
            signature_state: SignatureState::NotApplicable,
            attestation_state: AttestationState::NotApplicable,
            checksum_state: ChecksumState::ChecksumVerified,
            revocation_state: RevocationState::RevocationCurrent,
            revocation_freshness_class: FreshnessClass::Current,
            revocation_snapshot_ref: opt("revocation_snapshot:stable.2_1_0.current"),
            checked_at: opt("2026-04-29T18:49:00Z"),
            verification_evidence_refs: strings(&[
                "checksum:export.support_report.team_readiness.2026_04_29",
                "generated_lineage:export.support_report.team_readiness.2026_04_29",
                "revocation:none.2_1_0",
            ]),
            verification_disclosure:
                "Export checksum and generator lineage are recorded; product-build revocation is current, but the export is not a signed release artifact."
                    .to_owned(),
        },
        notice_inventory: NoticeInventory {
            license_state: LicenseState::LicenseAllowedWithNotice,
            license_expression_ref: opt("license_expression:export.support_report.team_readiness"),
            notice_state: NoticeState::NoticePartial,
            notice_inventory_state: NoticeInventoryState::InventoryPartial,
            notice_inventory_refs: strings(&[
                "notice_inventory:export.support_report.team_readiness.partial",
            ]),
            sbom_state: SbomState::NotApplicable,
            sbom_formats: vec![],
            notice_disclosure:
                "Export includes generated report content and a partial notice inventory for bundled template assets."
                    .to_owned(),
        },
        visible_cues: VisibleCues {
            source_cue: "Official generator, generated user artifact".to_owned(),
            provenance_cue:
                "Generated lineage names the exporter, product build, input summary refs, and output digest."
                    .to_owned(),
            license_cue: "License inventory is allowed with notice review for bundled report assets."
                .to_owned(),
            notice_cue: "Notice inventory is partial; redistribution requires review.".to_owned(),
            revocation_cue: "Product-build revocation metadata was current when the export was generated."
                .to_owned(),
            missing_or_partial_data: vec![
                missing_row(
                    DataClass::NoticeInventory,
                    MissingState::Partial,
                    "Partial notice inventory",
                    "The export carries notice refs for bundled template assets, but downstream redistribution requires review of the selected report body.",
                    opt("action:export_review.open_redistribution_hint.team_readiness"),
                ),
                missing_row(
                    DataClass::RedistributionTerms,
                    MissingState::Partial,
                    "Redistribution review required",
                    "The export is safe to save locally, but sharing outside the workspace requires reviewing notice and license refs.",
                    opt("action:export_review.open_redistribution_hint.team_readiness"),
                ),
            ],
        },
        access_points: vec![
            access_point(
                AccessPointClass::About,
                ReachabilityClass::AvailableReadOnly,
                opt("action:about.open_export_generator_build.team_readiness"),
                "About links the export back to the product build that generated it without calling the export an official release artifact.",
            ),
            access_point(
                AccessPointClass::UpdateCenter,
                ReachabilityClass::AvailableReadOnly,
                opt("action:update_center.inspect_generator_build.team_readiness"),
                "Update center can inspect the generator build identity and revocation state that applied at export time.",
            ),
            access_point(
                AccessPointClass::InstalledStateInspector,
                ReachabilityClass::Available,
                opt("action:installed_state.inspect.generated_export.team_readiness"),
                "Installed-state inspector identifies this as a generated user artifact.",
            ),
            access_point(
                AccessPointClass::DiagnosticsExport,
                ReachabilityClass::Available,
                opt("action:diagnostics.export_disclosure.generated_export.team_readiness"),
                "Diagnostics export preserves lineage refs, partial notice state, and redistribution hint.",
            ),
            access_point(
                AccessPointClass::ReviewSheet,
                ReachabilityClass::Available,
                opt("action:review_sheet.open.generated_export.team_readiness"),
                "Review sheet shows generated lineage and redistribution cues before external sharing.",
            ),
            access_point(
                AccessPointClass::GeneratedArtifactViewer,
                ReachabilityClass::Available,
                opt("action:generated_viewer.open_lineage.team_readiness"),
                "Generated-artifact viewer opens source inputs and output digest via the lineage record.",
            ),
            access_point(
                AccessPointClass::ExportReview,
                ReachabilityClass::Available,
                opt("action:export_review.open_redistribution_hint.team_readiness"),
                "Export review exposes the redistribution hint and required notice/license refs.",
            ),
        ],
        actions: vec![
            action(
                "action:generated_viewer.open_lineage.team_readiness",
                ActionClass::OpenGeneratedLineage,
                "Open lineage",
                opt("generated_lineage:export.support_report.team_readiness.2026_04_29"),
                ReachabilityClass::Available,
            ),
            action(
                "action:generated_viewer.open_source_inputs.team_readiness",
                ActionClass::OpenSourceInputs,
                "Open source inputs",
                opt("generated_lineage_inputs:export.support_report.team_readiness"),
                ReachabilityClass::Available,
            ),
            action(
                "action:export_review.open_redistribution_hint.team_readiness",
                ActionClass::OpenRedistributionHint,
                "Review redistribution",
                opt("redistribution_hint:export.support_report.team_readiness"),
                ReachabilityClass::Available,
            ),
            action(
                "action:diagnostics.export_disclosure.generated_export.team_readiness",
                ActionClass::ExportDisclosurePacket,
                "Export disclosure",
                opt("diagnostics_export:generated_export.team_readiness"),
                ReachabilityClass::Available,
            ),
        ],
        redistribution: Redistribution {
            redistribution_hint_class: RedistributionHintClass::ReviewBeforeRedistribution,
            redistribution_disclosure:
                "Export may be saved locally, but external redistribution requires reviewing partial notice inventory, license refs, and generated-lineage refs."
                    .to_owned(),
            required_notice_refs: strings(&[
                "notice_inventory:export.support_report.team_readiness.partial",
            ]),
            required_license_refs: strings(&["license_expression:export.support_report.team_readiness"]),
            required_lineage_refs: strings(&[
                "generated_lineage:export.support_report.team_readiness.2026_04_29",
            ]),
            policy_refs: strings(&["policy:export.redistribution_review"]),
        },
        linkage: Linkage {
            provenance_badge_refs: strings(&["provenance_badge:release.official.macos_arm64"]),
            update_manifest_refs: strings(&["update_manifest:stable.public.2_1_0"]),
            install_review_refs: vec![],
            extension_or_pack_review_refs: vec![],
            generated_lineage_refs: strings(&[
                "generated_lineage:export.support_report.team_readiness.2026_04_29",
            ]),
            release_evidence_refs: strings(&["release_evidence:stable.2_1_0"]),
            support_bundle_refs: strings(&["support_bundle:generated_export.team_readiness.metadata"]),
            diagnostics_export_refs: strings(&["diagnostics_export:generated_export.team_readiness"]),
            mirror_or_offline_receipt_refs: vec![],
        },
        export_projection: ExportProjection {
            diagnostics_export_refs: strings(&["diagnostics_export:generated_export.team_readiness"]),
            support_bundle_refs: strings(&["support_bundle:generated_export.team_readiness.metadata"]),
            public_proof_refs: vec![],
            offline_review_refs: vec![],
            redaction_class: RedactionClass::SupportRedacted,
            omission_reasons: strings(&[
                "Raw report body is omitted from metadata-only exports; lineage, digest, partial notice state, and redistribution hint remain.",
            ]),
        },
        narrative_refs: strings(&[
            "docs/governance/post_install_notice_and_provenance_contract.md",
            "schemas/workspace/generated_artifact_lineage.schema.json",
            "docs/generated/lineage_hint_packet.md",
        ]),
    }
}

fn honesty_invariants() -> DisclosureHonestyInvariants {
    DisclosureHonestyInvariants {
        subject_kind_explicit: true,
        missing_data_visible_not_omitted: true,
        source_class_and_subject_separate: true,
        trust_evidence_layered: true,
        post_install_access_survives: true,
        sbom_format_and_scope_explicit: true,
        exports_preserve_caveats: true,
        provenance_states_distinguish_official_mirrored_side_loaded_unknown: true,
        stale_or_revoked_never_reads_as_verified: true,
    }
}

fn consumer_projection() -> DisclosureConsumerProjection {
    DisclosureConsumerProjection {
        about_help_shows_disclosure_panel: true,
        installed_state_inspector_shows_disclosure_panel: true,
        diagnostics_export_includes_disclosure_record: true,
        marketplace_or_package_detail_shows_provenance_for_packs: true,
        does_not_replace_release_publication_artifacts: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        POST_INSTALL_DISCLOSURE_SCHEMA_REF,
        POST_INSTALL_DISCLOSURE_CONTRACT_REF,
        POST_INSTALL_PROVENANCE_BADGE_CONTRACT_REF,
        POST_INSTALL_PUBLIC_HANDOFF_MATRIX_REF,
        M5_POST_INSTALL_DISCLOSURE_PANEL_SET_SCHEMA_REF,
        M5_POST_INSTALL_DISCLOSURE_PANEL_SET_DOC_REF,
    ])
}

/// Returns the four canonical panels, one per governed artifact family.
pub fn seeded_post_install_panels() -> Vec<PostInstallDisclosureRecord> {
    vec![
        panel_desktop_official_signed_build(),
        panel_side_loaded_extension(),
        panel_mirrored_offline_artifact(),
        panel_generated_export(),
    ]
}

/// Builds the canonical stable M5 post-install disclosure panel set.
///
/// This is the single producer of the checked-in panel-set support export.
pub fn seeded_m5_post_install_disclosure_panel_set() -> M5PostInstallDisclosurePanelSet {
    M5PostInstallDisclosurePanelSet::new(M5PostInstallDisclosurePanelSetInput {
        packet_id: M5_POST_INSTALL_DISCLOSURE_PANEL_SET_ID.to_owned(),
        panel_set_label:
            "M5 Post-Install Notice/Provenance/SBOM Disclosure Panels for installed and generated artifact families"
                .to_owned(),
        panels: seeded_post_install_panels(),
        honesty_invariants: honesty_invariants(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}

/// Narrowed desktop-build panel whose signature was revoked after install.
///
/// The build no longer reads as verified: signature is revoked, revocation is
/// expired, and a visible revocation-snapshot row keeps the narrowing explicit.
pub fn seeded_post_install_product_build_signature_revoked() -> PostInstallDisclosureRecord {
    let mut panel = panel_desktop_official_signed_build();
    panel.disclosure_id = "post_install_disclosure:desktop.official.signature_revoked".to_owned();
    panel.verification.signature_state = SignatureState::SignatureRevoked;
    panel.verification.revocation_state = RevocationState::RevokedOrYanked;
    panel.verification.revocation_freshness_class = FreshnessClass::Expired;
    panel.verification.verification_disclosure =
        "The build's signing key was revoked after install; the signature no longer verifies and revocation is expired."
            .to_owned();
    panel.visible_cues.provenance_cue =
        "Signature is revoked for build build:release.stable.2_1_0; the build no longer reads as verified."
            .to_owned();
    panel.visible_cues.revocation_cue =
        "Revocation is expired and the artifact is yanked; reinstall a current signed build before trusting it."
            .to_owned();
    panel.visible_cues.missing_or_partial_data = vec![missing_row(
        DataClass::RevocationSnapshot,
        MissingState::Expired,
        "Revoked, revocation expired",
        "The signing key for this build was revoked and the local revocation snapshot is expired.",
        opt("action:update_center.refresh_revocation_snapshot.desktop.2_1_0"),
    )];
    panel.export_projection.omission_reasons = strings(&[
        "Public proof preserves revoked signature and expired revocation state; raw revocation payloads are omitted.",
    ]);
    panel
}

/// Narrowed generated-export panel whose SBOM is not provided.
///
/// Demonstrates honest "Not provided" SBOM surfacing for a generated artifact that
/// would otherwise be expected to carry one.
pub fn seeded_post_install_generated_export_sbom_not_provided() -> PostInstallDisclosureRecord {
    let mut panel = panel_generated_export();
    panel.disclosure_id =
        "post_install_disclosure:generated_export.support_report.sbom_not_provided".to_owned();
    panel.notice_inventory.sbom_state = SbomState::SbomMissing;
    panel.notice_inventory.sbom_formats = vec![];
    panel.notice_inventory.notice_disclosure =
        "Export bundles report content with a partial notice inventory; no SBOM was generated for the bundled assets."
            .to_owned();
    panel.visible_cues.notice_cue =
        "Notice inventory is partial and no SBOM is provided; redistribution requires review."
            .to_owned();
    panel.visible_cues.missing_or_partial_data.push(missing_row(
        DataClass::Sbom,
        MissingState::NotProvided,
        "SBOM not provided",
        "No SPDX or CycloneDX SBOM was generated for this export's bundled assets.",
        opt("action:export_review.open_redistribution_hint.team_readiness"),
    ));
    panel
}
