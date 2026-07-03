//! Canonical seed builders for the M5 disclosure / history block primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical disclosure-history-block-primitive packet.
pub const M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_PACKET_ID: &str =
    "m5-disclosure-history-block-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one worked resolution case from a fully specified disclosure input.
#[allow(clippy::too_many_arguments)]
fn disclosure_case(
    source_lane: M5DisclosureSourceLane,
    advisory_id: &str,
    cve_alias: &str,
    ghsa_alias: &str,
    severity: M5AdvisorySeverityClass,
    affected_object_repr: &str,
    current_status_repr: &str,
    history_state: M5DisclosureHistoryState,
    delivery_profile: M5AdvisoryDeliveryProfile,
    mirror_freshness: M5AdvisoryFreshnessState,
    disclosure_path_repr: &str,
    provenance_repr: &str,
    visibility_posture_repr: &str,
    action_state: M5AdvisoryActionState,
    continuity_claim: M5AdvisoryContinuityClaim,
) -> M5DisclosureResolutionCase {
    M5DisclosureResolutionCase::resolved(M5DisclosureBlockResolutionInput {
        source_lane,
        advisory_id: advisory_id.to_owned(),
        cve_alias: cve_alias.to_owned(),
        ghsa_alias: ghsa_alias.to_owned(),
        severity,
        affected_object_repr: affected_object_repr.to_owned(),
        current_status_repr: current_status_repr.to_owned(),
        history_state,
        delivery_profile,
        mirror_freshness,
        disclosure_path_repr: disclosure_path_repr.to_owned(),
        provenance_repr: provenance_repr.to_owned(),
        visibility_posture_repr: visibility_posture_repr.to_owned(),
        action_state,
        continuity_claim,
    })
}

/// A base row with the shared fields filled in and the full anatomy, severity, channel,
/// action, continuity, delivery, freshness, disclosure-field, history-state, focus,
/// export, and accessibility parity every lane carries. Parity is the guarantee: every
/// lane renders the same disclosure / history block model.
fn base_row(
    source_lane: M5DisclosureSourceLane,
    qualification: M5AdvisoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_disclosures: Vec<M5DisclosureResolutionCase>,
) -> M5DisclosureSourceRow {
    M5DisclosureSourceRow {
        source_lane,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // The disclosure / history block lives in the main workspace: the advisory-detail
        // surface where disclosure details and resolved-state history are inspected
        // without abandoning product context.
        shell_zone_slot: M5ShellZoneSlot::MainWorkspace,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5DisclosureBlockAnatomyPart::ALL.to_vec(),
        severity_classes: M5AdvisorySeverityClass::ALL.to_vec(),
        channels: M5DisclosureBlockChannel::ALL.to_vec(),
        action_states: M5AdvisoryActionState::ALL.to_vec(),
        required_actions: M5AdvisoryRequiredAction::ALL.to_vec(),
        continuity_claims: M5AdvisoryContinuityClaim::ALL.to_vec(),
        delivery_profiles: M5AdvisoryDeliveryProfile::ALL.to_vec(),
        freshness_states: M5AdvisoryFreshnessState::ALL.to_vec(),
        disclosure_fields: M5AdvisoryDisclosureField::ALL.to_vec(),
        history_states: M5DisclosureHistoryState::ALL.to_vec(),
        focus_behaviors: M5DisclosureBlockFocusBehavior::ALL.to_vec(),
        export_fields: M5AdvisoryExportField::ALL.to_vec(),
        accessibility_routes: M5AdvisoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ReleaseProof,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5AdvisoryDowngradeTrigger::AffectedScopeHidden,
            M5AdvisoryDowngradeTrigger::LocalContinuityHidden,
            M5AdvisoryDowngradeTrigger::MirrorLagUndisclosed,
            M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent,
            M5AdvisoryDowngradeTrigger::ExternalDisclosureOnly,
            M5AdvisoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DISCLOSURE_HISTORY_BLOCK_SCHEMA_REF,
            M5_DISCLOSURE_HISTORY_BLOCK_IDENTITY_REF,
            M5_DISCLOSURE_HISTORY_BLOCK_HISTORY_DOC_REF,
            M5_DISCLOSURE_HISTORY_BLOCK_POSTMORTEM_DOC_REF,
        ]),
        example_disclosures,
        flattens_disclosure_into_external_link: false,
        hides_field_behind_detail_drawer: false,
        drops_resolved_history_from_inspection: false,
        hides_provenance_when_mirrored_or_external: false,
        drops_copy_safe_id_or_export: false,
    }
}

fn source_rows() -> Vec<M5DisclosureSourceRow> {
    use M5AdvisoryActionState as A;
    use M5AdvisoryContinuityClaim as C;
    use M5AdvisoryDeliveryProfile as D;
    use M5AdvisoryFreshnessState as F;
    use M5AdvisorySeverityClass as S;
    use M5DisclosureHistoryState as H;

    let mut rows = Vec::new();

    // 1. First-party signed — critical, published: the active first-party disclosure keeps
    //    full weight, carries both a CVE and a GHSA alias as copy-safe ids, and hands off
    //    to a bundled in-product doc (no external navigation).
    rows.push(base_row(
        M5DisclosureSourceLane::FirstPartySigned,
        M5AdvisoryQualificationClass::Stable,
        "Security-advisory / disclosure owner",
        "The first-party-signed lane renders a published critical disclosure at full weight with the current status, affected versions, the copy-safe Aureline / CVE / GHSA reference ids, and an open-in-product-doc handoff — the copy-safe ids never degrade to a link",
        "evidence:m5-disclosure-first-party:001",
        vec![disclosure_case(
            M5DisclosureSourceLane::FirstPartySigned,
            "AURELINE-ADV-2026-0401",
            "CVE-2026-0401",
            "GHSA-q7r8-s9t0-u1v2",
            S::Critical,
            "affected_versions:2026.6.0-2026.6.0",
            "current_status:published_action_required",
            H::Published,
            D::LocalOnly,
            F::UpToDate,
            "disclosure_path:advisory_db_local_signed",
            "provenance:first_party_signed_current",
            "visibility:public_published",
            A::ActionRequired,
            C::DegradedLocalMode,
        )],
    ));

    // 2. Mirrored — operational emergency, mitigated: an active advisory delivered through
    //    an approved mirror. It keeps full weight (active response chain), discloses the
    //    mirror freshness, and preserves the mirror provenance on handoff.
    rows.push(base_row(
        M5DisclosureSourceLane::Mirrored,
        M5AdvisoryQualificationClass::Stable,
        "Mirror / offline-continuity owner",
        "The mirrored lane renders a mitigated operational-emergency disclosure delivered through an approved mirror, disclosing the mirror freshness and preserving the mirror provenance so the handoff never becomes a dead-end link",
        "evidence:m5-disclosure-mirrored:001",
        vec![disclosure_case(
            M5DisclosureSourceLane::Mirrored,
            "AURELINE-ADV-2026-0402",
            "CVE-2026-0402",
            "",
            S::OperationalEmergency,
            "affected_versions:2026.5.0-2026.6.0",
            "current_status:mitigated_active_response",
            H::Mitigated,
            D::OfflineMirror,
            F::StaleWithinGrace,
            "disclosure_path:advisory_db_mirror_snapshot",
            "provenance:mirror_signed_within_grace",
            "visibility:public_mirrored",
            A::ImmediateRemediation,
            C::OfflineMirrorLagDisclosed,
        )],
    ));

    // 3. Offline imported — high, superseded: a manually imported disclosure that has been
    //    superseded. It steps down to inspectable history, keeps the offline-import
    //    provenance, and discloses that the offline snapshot is expired.
    rows.push(base_row(
        M5DisclosureSourceLane::OfflineImported,
        M5AdvisoryQualificationClass::Stable,
        "Offline-import / bundle owner",
        "The offline-imported lane renders a superseded high disclosure stepped down to inspectable history while keeping the current-status truth, the offline-import provenance, and the expired-snapshot disclosure",
        "evidence:m5-disclosure-offline-imported:001",
        vec![disclosure_case(
            M5DisclosureSourceLane::OfflineImported,
            "AURELINE-ADV-2026-0403",
            "CVE-2026-0403",
            "",
            S::High,
            "affected_versions:2026.4.0-2026.5.0",
            "current_status:superseded_by_fixed_advisory",
            H::Superseded,
            D::ManualImport,
            F::OfflineExpired,
            "disclosure_path:advisory_db_offline_bundle",
            "provenance:offline_import_signed_snapshot",
            "visibility:public_superseded_history",
            A::MitigationComplete,
            C::ContinuityPendingFix,
        )],
    ));

    // 4. Externally linked — moderate, resolved: a disclosure that links out to an external
    //    page. It steps down to inspectable history, opens the external browser while
    //    preserving provenance and the in-product state, and carries a GHSA alias.
    rows.push(base_row(
        M5DisclosureSourceLane::ExternallyLinked,
        M5AdvisoryQualificationClass::Stable,
        "External-disclosure liaison",
        "The externally-linked lane renders a resolved moderate disclosure stepped down to inspectable history and opens an external browser while preserving provenance — it never replaces the in-product disclosure state with a dead-end link",
        "evidence:m5-disclosure-externally-linked:001",
        vec![disclosure_case(
            M5DisclosureSourceLane::ExternallyLinked,
            "AURELINE-ADV-2026-0404",
            "",
            "GHSA-a1b2-c3d4-e5f6",
            S::Moderate,
            "affected_versions:2026.3.0-2026.4.0",
            "current_status:resolved_fixed_build_promoted",
            H::Resolved,
            D::Managed,
            F::UpToDate,
            "disclosure_path:external_disclosure_reference",
            "provenance:external_authority_referenced",
            "visibility:public_resolved_history",
            A::MitigationComplete,
            C::LocalUseUnaffected,
        )],
    ));

    // 5. Community postmortem — low, withdrawn: a community postmortem cross-reference for
    //    a withdrawn advisory. It stays visible as a withdrawn history row, stepped down
    //    but inspectable, and preserves the external provenance.
    rows.push(base_row(
        M5DisclosureSourceLane::CommunityPostmortem,
        M5AdvisoryQualificationClass::Stable,
        "Community-handoff owner",
        "The community-postmortem lane renders a withdrawn low disclosure that remains visible as a stepped-down but inspectable withdrawn history row and preserves the community-postmortem provenance instead of silently removing the row",
        "evidence:m5-disclosure-community-postmortem:001",
        vec![disclosure_case(
            M5DisclosureSourceLane::CommunityPostmortem,
            "AURELINE-ADV-2026-0405",
            "",
            "",
            S::Low,
            "affected_versions:2026.2.0-2026.2.0",
            "current_status:withdrawn_not_a_security_issue",
            H::Withdrawn,
            D::LocalOnly,
            F::UpToDate,
            "disclosure_path:community_postmortem_reference",
            "provenance:community_postmortem_referenced",
            "visibility:public_withdrawn_history",
            A::Informational,
            C::LocalUseUnaffected,
        )],
    ));

    // 6. Vendor cross-reference — informational, draft: an upstream vendor cross-reference
    //    still in draft. It renders at restricted draft weight while keeping the copy-safe
    //    ids and the external vendor provenance.
    rows.push(base_row(
        M5DisclosureSourceLane::VendorCrossReference,
        M5AdvisoryQualificationClass::Stable,
        "Upstream / vendor liaison",
        "The vendor-cross-reference lane renders an informational draft disclosure at restricted draft weight, keeping the copy-safe reference ids and the upstream-vendor provenance visible while the draft is not yet published",
        "evidence:m5-disclosure-vendor-cross-reference:001",
        vec![disclosure_case(
            M5DisclosureSourceLane::VendorCrossReference,
            "AURELINE-ADV-2026-0406",
            "CVE-2026-0406",
            "",
            S::Informational,
            "affected_versions:pending_confirmation",
            "current_status:draft_pending_publication",
            H::Draft,
            D::Managed,
            F::Unknown,
            "disclosure_path:vendor_cross_reference",
            "provenance:upstream_vendor_referenced",
            "visibility:restricted_draft",
            A::ReviewRecommended,
            C::ContinuityPendingFix,
        )],
    ));

    rows
}

fn governance_review() -> M5DisclosureBlockGovernanceReview {
    M5DisclosureBlockGovernanceReview {
        one_block_model_across_source_lanes: true,
        current_status_versions_path_visible_without_drawer: true,
        reference_ids_are_copy_safe: true,
        open_doc_and_open_browser_actions_present: true,
        resolved_advisories_step_down_but_remain_inspectable: true,
        provenance_visible_when_mirrored_offline_or_external: true,
        external_handoff_preserves_in_product_state: true,
        copy_safe_advisory_id_preserved: true,
        export_summary_reconstructs_disclosure_truth: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DisclosureBlockConsumerProjection {
    M5DisclosureBlockConsumerProjection {
        help_about_renders_shared_block: true,
        update_center_renders_shared_block: true,
        support_bundle_renders_shared_block: true,
        history_view_reads_single_source: true,
        resolver_reads_single_disclosure_vocabulary: true,
    }
}

fn proof_freshness() -> M5DisclosureBlockProofFreshness {
    M5DisclosureBlockProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DisclosureBlockReleasePosture {
    M5DisclosureBlockReleasePosture {
        release_packet_ref: M5_DISCLOSURE_HISTORY_BLOCK_ARTIFACT_REF.to_owned(),
        disclosure_audit_ref: M5_DISCLOSURE_HISTORY_BLOCK_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DISCLOSURE_HISTORY_BLOCK_SCHEMA_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_DOC_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_SHELL_ZONE_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_COMPONENT_MATRIX_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_IDENTITY_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_HISTORY_DOC_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_POSTMORTEM_DOC_REF,
    ])
}

/// Builds the canonical M5 disclosure-history-block-primitive packet.
pub fn seeded_m5_disclosure_history_block_primitive_packet() -> M5DisclosureHistoryBlockPacket {
    M5DisclosureHistoryBlockPacket::new(M5DisclosureHistoryBlockPacketInput {
        packet_id: M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 disclosure / history block primitive: current status, affected versions / components, copy-safe CVE / GHSA reference ids, resolved-state downgrade, provenance, and open-doc / open-browser parity across Help/About, update, and support channels"
                .to_owned(),
        source_rows: source_rows(),
        vocabulary_set: M5DisclosureBlockVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the offline-imported lane is held at Beta because a slice of the
/// expired-snapshot history projection does not yet render on every offline profile;
/// every lane stays visible.
pub fn seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed(
) -> M5DisclosureHistoryBlockPacket {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.packet_id =
        "m5-disclosure-history-block-primitive:offline-imported-beta:0001".to_owned();
    let row = packet
        .source_rows
        .iter_mut()
        .find(|row| row.source_lane == M5DisclosureSourceLane::OfflineImported)
        .expect("offline-imported row present");
    row.qualification = M5AdvisoryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the externally-linked lane is narrowed to Preview pending
/// provenance-preservation parity across every external handoff; every lane stays visible.
pub fn seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed(
) -> M5DisclosureHistoryBlockPacket {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.packet_id =
        "m5-disclosure-history-block-primitive:externally-linked-preview:0001".to_owned();
    let row = packet
        .source_rows
        .iter_mut()
        .find(|row| row.source_lane == M5DisclosureSourceLane::ExternallyLinked)
        .expect("externally-linked row present");
    row.qualification = M5AdvisoryQualificationClass::Preview;
    packet
}
