use super::*;

use std::collections::BTreeSet;

use crate::m5_author_and_publish_preview::current_m5_author_publish_matrix;

fn packet() -> M5PublishPreviewSheetSet {
    current_m5_publish_preview_sheet_set().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_PUBLISH_PREVIEW_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_PUBLISH_PREVIEW_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_sheets() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_sheet() {
    let packet = packet();
    assert_eq!(packet.sheets.len(), packet.artifact_families.len());
    for &family in &packet.artifact_families {
        assert!(
            packet.sheet(family).is_some(),
            "missing sheet for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_sheet_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_sheets_consistent());
    for sheet in &packet.sheets {
        assert_eq!(
            sheet.published_trust_posture,
            sheet.effective_trust_posture(),
            "sheet {} publishes beyond the signing/namespace ceiling",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.publish_readiness,
            sheet.computed_publish_readiness(),
            "sheet {} readiness diverges from the gate",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.findings,
            sheet.computed_findings(),
            "sheet {} findings diverge from the gate",
            sheet.sheet_id
        );
    }
}

#[test]
fn every_sheet_carries_its_own_refs_and_all_checks() {
    let packet = packet();
    for sheet in &packet.sheets {
        assert!(
            sheet.has_required_refs(),
            "sheet {} is missing required refs",
            sheet.sheet_id
        );
        for check in PublishCheck::ALL {
            assert!(
                sheet.check(check).is_some(),
                "sheet {} is missing check {}",
                sheet.sheet_id,
                check.as_str()
            );
        }
    }
}

#[test]
fn findings_are_in_canonical_order_with_consistent_severity_and_source() {
    let packet = packet();
    for sheet in &packet.sheets {
        let keys: Vec<(u8, u8)> = sheet
            .findings
            .iter()
            .map(|f| (f.source.rank(), f.reason.rank()))
            .collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        assert_eq!(
            keys, sorted,
            "sheet {} findings out of order",
            sheet.sheet_id
        );
        for finding in &sheet.findings {
            assert_eq!(
                finding.severity,
                finding.reason.severity(),
                "sheet {} finding {} has the wrong severity",
                sheet.sheet_id,
                finding.reason.as_str()
            );
            assert!(
                finding.reason.source_is_valid(finding.source),
                "sheet {} finding {} mis-attributes its source",
                sheet.sheet_id,
                finding.reason.as_str()
            );
        }
    }
}

#[test]
fn blockers_versus_warnings_stay_explicit() {
    let packet = packet();
    for sheet in &packet.sheets {
        match sheet.publish_readiness {
            PublishReadiness::PublishableWithWarnings => {
                assert!(sheet.warning_count() > 0, "{}", sheet.sheet_id);
                assert_eq!(sheet.blocker_count(), 0, "{}", sheet.sheet_id);
            }
            PublishReadiness::BlockedFromPublish => {
                assert!(sheet.blocker_count() > 0, "{}", sheet.sheet_id);
            }
            PublishReadiness::ReadyToPublish => {
                assert_eq!(sheet.blocker_count(), 0, "{}", sheet.sheet_id);
                assert_eq!(sheet.warning_count(), 0, "{}", sheet.sheet_id);
            }
            PublishReadiness::WithheldQuarantined => {
                assert!(
                    sheet.anti_abuse_transparency.is_quarantined(),
                    "{}",
                    sheet.sheet_id
                );
            }
        }
    }
}

#[test]
fn ready_sheet_is_genuinely_clean() {
    let packet = packet();
    assert!(
        packet.ready_sheets().count() > 0,
        "fixture needs a ready-to-publish sheet"
    );
    for sheet in packet.ready_sheets() {
        assert_eq!(sheet.signature_state, SignatureState::SignedVerified);
        assert!(!sheet.namespace_state.caps_to_local_only());
        assert_eq!(
            sheet.anti_abuse_transparency,
            AntiAbuseTransparency::DisclosedClean
        );
        assert!(!sheet.has_widening_change() || sheet.widening_reviewed);
        assert!(sheet.findings.is_empty());
        assert!(sheet.published_trust_posture.is_trusted_badge());
    }
}

#[test]
fn named_checks_attribute_their_findings_to_the_right_source() {
    // A blocked or warned named check raises a finding whose source is that check, so a
    // reviewer can always see which gate produced a blocker.
    let packet = packet();
    let mut proved = 0;
    for sheet in &packet.sheets {
        for result in &sheet.checks {
            match result.outcome {
                CheckOutcome::Blocked => {
                    assert!(
                        sheet
                            .findings
                            .iter()
                            .any(|f| f.source == result.check.source()
                                && f.reason == FindingReason::CheckFailed),
                        "sheet {} blocked check {} has no blocking finding",
                        sheet.sheet_id,
                        result.check.as_str()
                    );
                    proved += 1;
                }
                CheckOutcome::Warning => {
                    assert!(sheet
                        .findings
                        .iter()
                        .any(|f| f.source == result.check.source()
                            && f.reason == FindingReason::CheckWarning));
                    proved += 1;
                }
                CheckOutcome::NotRun => {
                    assert!(sheet
                        .findings
                        .iter()
                        .any(|f| f.source == result.check.source()
                            && f.reason == FindingReason::CheckNotRun));
                    proved += 1;
                }
                CheckOutcome::Passed | CheckOutcome::NotApplicable => {
                    assert!(!sheet
                        .findings
                        .iter()
                        .any(|f| f.source == result.check.source()));
                }
            }
        }
    }
    assert!(
        proved >= 5,
        "fixture must exercise multiple named-check findings"
    );
}

#[test]
fn version_bump_must_cover_the_change_impact() {
    let packet = packet();
    let mut proved_missing = false;
    let mut proved_undersized = false;
    let mut proved_downgrade = false;
    let mut proved_invalid = false;
    for sheet in &packet.sheets {
        match sheet.version_bump.finding(sheet.max_change_impact()) {
            Some(FindingReason::VersionBumpMissing) => {
                assert!(sheet.has_blocker_from(FindingSource::VersionBump));
                proved_missing = true;
            }
            Some(FindingReason::VersionBumpUndersized) => {
                assert!(sheet.has_blocker_from(FindingSource::VersionBump));
                proved_undersized = true;
            }
            Some(FindingReason::VersionDowngrade) => proved_downgrade = true,
            Some(FindingReason::VersionInvalid) => proved_invalid = true,
            _ => {}
        }
    }
    assert!(proved_missing, "no sheet exercises a missing version bump");
    assert!(
        proved_undersized,
        "no sheet exercises an undersized version bump"
    );
    assert!(proved_downgrade, "no sheet exercises a version downgrade");
    assert!(proved_invalid, "no sheet exercises an invalid version");
}

#[test]
fn widening_changes_require_a_fresh_review() {
    let packet = packet();
    let mut proved = 0;
    for sheet in &packet.sheets {
        let manifest_widens = sheet.manifest_deltas.iter().any(|d| d.kind.is_widening());
        let hot_reload_widens = hot_reload_widens_authority(sheet.hot_reload_posture);
        if (manifest_widens || hot_reload_widens) && !sheet.widening_reviewed {
            assert_ne!(
                sheet.publish_readiness,
                PublishReadiness::ReadyToPublish,
                "sheet {} widens authority but is still ready to publish",
                sheet.sheet_id
            );
            if manifest_widens {
                assert!(sheet
                    .findings
                    .iter()
                    .any(|f| f.reason == FindingReason::ManifestWideningUnreviewed));
            }
            if hot_reload_widens {
                assert!(sheet
                    .findings
                    .iter()
                    .any(|f| f.reason == FindingReason::HotReloadWideningUnreviewed));
            }
            proved += 1;
        }
    }
    assert!(
        proved >= 1,
        "fixture must exercise the widening-review guardrail"
    );
}

#[test]
fn local_or_untrusted_artifacts_never_inherit_trusted_badges() {
    let packet = packet();
    let mut proved = 0;
    for sheet in &packet.sheets {
        if sheet.signature_state.is_local_or_untrusted()
            || sheet.namespace_state.caps_to_local_only()
        {
            assert_eq!(
                sheet.published_trust_posture,
                TrustPosture::UnsignedLocalOnly,
                "sheet {} inherited a trusted badge from a local/untrusted source",
                sheet.sheet_id
            );
            proved += 1;
        }
    }
    assert!(
        proved >= 1,
        "fixture must exercise the non-inheritance guardrail"
    );
}

#[test]
fn signed_artifact_with_unowned_namespace_is_capped() {
    // The recipe pack is signed-verified, but its namespace is mid-transfer, so the
    // published badge is capped to registry-bound rather than verified-publisher.
    let packet = packet();
    let sheet = packet
        .sheet(ArtifactFamily::SignedRecipePack)
        .expect("recipe sheet");
    assert_eq!(sheet.signature_state, SignatureState::SignedVerified);
    assert_eq!(
        sheet.namespace_state,
        NamespaceState::NamespaceTransferPending
    );
    assert_eq!(
        sheet.declared_trust_posture,
        TrustPosture::VerifiedPublisher
    );
    assert_eq!(sheet.published_trust_posture, TrustPosture::RegistryBound);
}

#[test]
fn channel_selection_consequences_are_explicit() {
    let packet = packet();
    let mut proved_signed = false;
    let mut proved_clean = false;
    for sheet in &packet.sheets {
        if sheet.release_channel.requires_signed_release()
            && sheet.effective_trust_posture().rank() < TrustPosture::RegistryBound.rank()
        {
            assert!(
                sheet
                    .findings
                    .iter()
                    .any(|f| f.reason == FindingReason::ChannelRequiresSignedRelease),
                "sheet {} on a signed-release channel is unsigned but raises no channel finding",
                sheet.sheet_id
            );
            proved_signed = true;
        }
        if sheet.release_channel.requires_clean_release() && sheet.warning_count() > 0 {
            assert!(
                sheet
                    .findings
                    .iter()
                    .any(|f| f.reason == FindingReason::ChannelRequiresCleanRelease),
                "sheet {} on a clean-release channel still carries warnings but raises no channel finding",
                sheet.sheet_id
            );
            proved_clean = true;
        }
    }
    assert!(
        proved_signed,
        "fixture must exercise the signed-release channel guard"
    );
    assert!(
        proved_clean,
        "fixture must exercise the clean-release channel guard"
    );
}

#[test]
fn quarantined_sheet_is_withheld() {
    let packet = packet();
    let quarantined: Vec<_> = packet
        .sheets
        .iter()
        .filter(|s| s.anti_abuse_transparency.is_quarantined())
        .collect();
    assert!(!quarantined.is_empty(), "fixture needs a quarantined sheet");
    for sheet in quarantined {
        assert_eq!(
            sheet.publish_readiness,
            PublishReadiness::WithheldQuarantined
        );
        assert!(sheet
            .findings
            .iter()
            .any(|f| f.reason == FindingReason::AntiAbuseQuarantined));
    }
}

#[test]
fn cross_check_matrix_agrees_with_publish_gate() {
    let packet = packet();
    let matrix = current_m5_author_publish_matrix().expect("matrix parses");
    assert_eq!(packet.cross_check_matrix(&matrix), Vec::new());
    for sheet in &packet.sheets {
        let row = matrix
            .family(sheet.artifact_family)
            .expect("matrix row for family");
        assert!(
            sheet.published_trust_posture.rank() <= row.published_trust_posture.rank(),
            "sheet {} publishes stronger than the gate",
            sheet.sheet_id
        );
    }
}

#[test]
fn export_projection_reflects_the_sheets() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.sheets.len(), packet.sheets.len());
    assert!(projection.all_sheets_consistent);
    assert_eq!(projection.ready_count, packet.ready_sheets().count());
    assert_eq!(
        projection.blocked_or_withheld_count,
        packet.blocked_or_withheld_sheets().count()
    );
    assert_eq!(
        projection.local_only_count,
        packet.local_only_sheets().count()
    );
    for (row, sheet) in projection.sheets.iter().zip(&packet.sheets) {
        assert_eq!(row.sheet_id, sheet.sheet_id);
        assert_eq!(row.blocker_count, sheet.blocker_count());
        assert_eq!(row.warning_count, sheet.warning_count());
        assert_eq!(row.publish_ready, sheet.is_ready_to_publish());
        assert_eq!(row.blocker_sources.len(), sheet.blocker_count());
        assert_eq!(row.warning_sources.len(), sheet.warning_count());
    }
}

#[test]
fn release_channels_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ReleaseChannel> =
        packet.sheets.iter().map(|s| s.release_channel).collect();
    for channel in ReleaseChannel::ALL {
        assert!(
            present.contains(&channel),
            "no sheet exercises release channel {}",
            channel.as_str()
        );
    }
}

#[test]
fn namespace_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<NamespaceState> =
        packet.sheets.iter().map(|s| s.namespace_state).collect();
    for state in NamespaceState::ALL {
        assert!(
            present.contains(&state),
            "no sheet exercises namespace state {}",
            state.as_str()
        );
    }
}

#[test]
fn version_bumps_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<VersionBump> = packet.sheets.iter().map(|s| s.version_bump).collect();
    for bump in VersionBump::ALL {
        assert!(
            present.contains(&bump),
            "no sheet exercises version bump {}",
            bump.as_str()
        );
    }
}

#[test]
fn change_impacts_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ChangeImpact> = packet
        .sheets
        .iter()
        .flat_map(|s| s.manifest_deltas.iter().map(|d| d.kind.change_impact()))
        .collect();
    for impact in ChangeImpact::ALL {
        assert!(
            present.contains(&impact),
            "no manifest delta exercises change impact {}",
            impact.as_str()
        );
    }
}

#[test]
fn manifest_delta_kinds_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ManifestDeltaKind> = packet
        .sheets
        .iter()
        .flat_map(|s| s.manifest_deltas.iter().map(|d| d.kind))
        .collect();
    for kind in ManifestDeltaKind::ALL {
        assert!(
            present.contains(&kind),
            "no sheet exercises manifest delta kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn check_outcomes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CheckOutcome> = packet
        .sheets
        .iter()
        .flat_map(|s| s.checks.iter().map(|c| c.outcome))
        .collect();
    for outcome in CheckOutcome::ALL {
        assert!(
            present.contains(&outcome),
            "no check exercises outcome {}",
            outcome.as_str()
        );
    }
}

#[test]
fn signature_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SignatureState> =
        packet.sheets.iter().map(|s| s.signature_state).collect();
    for state in SignatureState::ALL {
        assert!(
            present.contains(&state),
            "no sheet exercises signing state {}",
            state.as_str()
        );
    }
}

#[test]
fn published_trust_postures_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<TrustPosture> = packet
        .sheets
        .iter()
        .map(|s| s.published_trust_posture)
        .collect();
    for posture in TrustPosture::ALL {
        assert!(
            present.contains(&posture),
            "no sheet publishes trust posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn anti_abuse_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AntiAbuseTransparency> = packet
        .sheets
        .iter()
        .map(|s| s.anti_abuse_transparency)
        .collect();
    for state in AntiAbuseTransparency::ALL {
        assert!(
            present.contains(&state),
            "no sheet exercises anti-abuse state {}",
            state.as_str()
        );
    }
}

#[test]
fn publish_readiness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PublishReadiness> =
        packet.sheets.iter().map(|s| s.publish_readiness).collect();
    for readiness in PublishReadiness::ALL {
        assert!(
            present.contains(&readiness),
            "no sheet exercises publish readiness {}",
            readiness.as_str()
        );
    }
}

#[test]
fn finding_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FindingReason> = packet
        .sheets
        .iter()
        .flat_map(|s| s.findings.iter().map(|f| f.reason))
        .collect();
    for reason in FindingReason::ALL {
        assert!(
            present.contains(&reason),
            "no sheet exercises finding reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn finding_sources_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FindingSource> = packet
        .sheets
        .iter()
        .flat_map(|s| s.findings.iter().map(|f| f.source))
        .collect();
    for source in FindingSource::ALL {
        assert!(
            present.contains(&source),
            "no sheet exercises finding source {}",
            source.as_str()
        );
    }
}

#[test]
fn both_severities_are_exercised() {
    let packet = packet();
    let present: BTreeSet<FindingSeverity> = packet
        .sheets
        .iter()
        .flat_map(|s| s.findings.iter().map(|f| f.severity))
        .collect();
    for severity in FindingSeverity::ALL {
        assert!(
            present.contains(&severity),
            "no finding carries severity {}",
            severity.as_str()
        );
    }
}

#[test]
fn validate_flags_overstated_trust_posture() {
    let mut packet = packet();
    if let Some(sheet) = packet
        .sheets
        .iter_mut()
        .find(|s| s.effective_trust_posture() != TrustPosture::EnterpriseApproved)
    {
        sheet.published_trust_posture = TrustPosture::EnterpriseApproved;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5PublishPreviewViolation::OverstatedTrustPosture { .. })));
    }
}

#[test]
fn validate_flags_local_artifact_inherited_trust() {
    let mut packet = packet();
    let idx = packet
        .sheets
        .iter()
        .position(|s| s.artifact_family == ArtifactFamily::SideLoadedPackage)
        .expect("side-loaded sheet");
    packet.sheets[idx].published_trust_posture = TrustPosture::VerifiedPublisher;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5PublishPreviewViolation::LocalArtifactInheritedTrust { .. }
    )));
}

#[test]
fn validate_flags_findings_mismatch() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.findings.is_empty())
        .expect("a clean sheet");
    sheet.findings.push(PublishPreviewFinding::of(
        FindingSource::SchemaValidation,
        FindingReason::CheckFailed,
        "x".to_owned(),
    ));
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5PublishPreviewViolation::FindingsMismatch { .. })));
}

#[test]
fn validate_flags_finding_source_mismatch() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| !s.findings.is_empty())
        .expect("a sheet with findings");
    // Attribute a version finding to a namespace source.
    sheet.findings[0].source = FindingSource::Namespace;
    sheet.findings[0].reason = FindingReason::VersionInvalid;
    sheet.findings[0].severity = FindingReason::VersionInvalid.severity();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5PublishPreviewViolation::FindingSourceMismatch { .. })));
}

#[test]
fn validate_flags_missing_check() {
    let mut packet = packet();
    packet.sheets[0]
        .checks
        .retain(|c| c.check != PublishCheck::RegistryPolicy);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5PublishPreviewViolation::MissingCheck { .. })));
}

#[test]
fn validate_flags_readiness_mismatch() {
    let mut packet = packet();
    if let Some(sheet) = packet
        .sheets
        .iter_mut()
        .find(|s| s.publish_readiness != PublishReadiness::WithheldQuarantined)
    {
        sheet.publish_readiness = PublishReadiness::WithheldQuarantined;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5PublishPreviewViolation::ReadinessMismatch { .. })));
    }
}

#[test]
fn validate_flags_missing_family_sheet() {
    let mut packet = packet();
    let removed = packet.sheets.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5PublishPreviewViolation::MissingFamilySheet { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_sheets = packet.summary.total_sheets.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5PublishPreviewViolation::SummaryMismatch));
}

#[test]
fn cross_check_flags_sheet_exceeding_gate() {
    let mut packet = packet();
    let matrix = current_m5_author_publish_matrix().expect("matrix parses");
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.artifact_family == ArtifactFamily::SideLoadedPackage)
        .expect("side-loaded sheet");
    sheet.published_trust_posture = TrustPosture::EnterpriseApproved;
    let violations = packet.cross_check_matrix(&matrix);
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5PublishPreviewViolation::SheetExceedsPublishGate { .. })));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        M5_PUBLISH_PREVIEW_RECORD_KIND,
        "m5_publish_preview_sheet_set"
    );
    assert_eq!(ReleaseChannel::Stable.as_str(), "stable");
    assert_eq!(
        NamespaceState::NamespaceTransferPending.as_str(),
        "namespace_transfer_pending"
    );
    assert_eq!(VersionBump::NoBump.as_str(), "no_bump");
    assert_eq!(ChangeImpact::Breaking.as_str(), "breaking");
    assert_eq!(
        ManifestDeltaKind::ExternalExecutableAdded.as_str(),
        "external_executable_added"
    );
    assert_eq!(
        PublishCheck::TemplateSampleCompleteness.as_str(),
        "template_sample_completeness"
    );
    assert_eq!(CheckOutcome::NotRun.as_str(), "not_run");
    assert_eq!(FindingSource::HotReloadReview.as_str(), "hot_reload_review");
    assert_eq!(
        FindingReason::ChannelRequiresCleanRelease.as_str(),
        "channel_requires_clean_release"
    );
}

#[test]
fn version_bump_covers_impact_table() {
    assert!(VersionBump::Major.covers_impact(ChangeImpact::Breaking));
    assert!(!VersionBump::Minor.covers_impact(ChangeImpact::Breaking));
    assert!(VersionBump::Minor.covers_impact(ChangeImpact::Feature));
    assert!(!VersionBump::Patch.covers_impact(ChangeImpact::Feature));
    assert!(VersionBump::Patch.covers_impact(ChangeImpact::Fix));
    assert!(!VersionBump::NoBump.covers_impact(ChangeImpact::Fix));
    assert!(VersionBump::NoBump.covers_impact(ChangeImpact::NoImpact));
    assert!(!VersionBump::Downgrade.covers_impact(ChangeImpact::NoImpact));
    assert!(!VersionBump::Invalid.covers_impact(ChangeImpact::NoImpact));
}

#[test]
fn namespace_trust_ceilings_hold() {
    assert_eq!(
        NamespaceState::EnterpriseManaged.trust_ceiling(),
        TrustPosture::EnterpriseApproved
    );
    assert_eq!(
        NamespaceState::PublisherVerified.trust_ceiling(),
        TrustPosture::VerifiedPublisher
    );
    assert_eq!(
        NamespaceState::PublisherOwned.trust_ceiling(),
        TrustPosture::RegistryBound
    );
    assert_eq!(
        NamespaceState::NamespaceMismatch.trust_ceiling(),
        TrustPosture::UnsignedLocalOnly
    );
    assert_eq!(
        NamespaceState::NamespaceUnclaimed.trust_ceiling(),
        TrustPosture::UnsignedLocalOnly
    );
}
