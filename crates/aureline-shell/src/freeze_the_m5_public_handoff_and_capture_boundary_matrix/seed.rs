//! Canonical seed builders for the frozen M5 public-handoff / capture-boundary
//! matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical public-handoff matrix.
pub const M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID: &str = "m5-public-handoff-matrix:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn tokens_for(vocab: M5HandoffStateVocabulary) -> HandoffTokenVecs {
    use M5HandoffStateVocabulary as V;
    let mut vecs = HandoffTokenVecs::default();
    match vocab {
        V::ProvenanceClass => vecs.provenance_classes = HandoffProvenanceClass::ALL.to_vec(),
        V::RouteTrustClass => vecs.route_trust_classes = HandoffRouteTrustClass::ALL.to_vec(),
        V::CapturePermissionState => {
            vecs.capture_permission_states = HandoffCapturePermissionState::ALL.to_vec()
        }
        V::RedactionState => vecs.redaction_states = HandoffRedactionState::ALL.to_vec(),
        V::ContinuityState => vecs.continuity_states = HandoffContinuityState::ALL.to_vec(),
        V::BoundaryChromeHonesty => {
            vecs.boundary_chrome_states = HandoffBoundaryChromeHonesty::ALL.to_vec()
        }
        V::NoticeFreshnessState => {
            vecs.notice_freshness_states = HandoffNoticeFreshnessState::ALL.to_vec()
        }
    }
    vecs
}

#[derive(Default)]
struct HandoffTokenVecs {
    provenance_classes: Vec<HandoffProvenanceClass>,
    route_trust_classes: Vec<HandoffRouteTrustClass>,
    capture_permission_states: Vec<HandoffCapturePermissionState>,
    redaction_states: Vec<HandoffRedactionState>,
    continuity_states: Vec<HandoffContinuityState>,
    boundary_chrome_states: Vec<HandoffBoundaryChromeHonesty>,
    notice_freshness_states: Vec<HandoffNoticeFreshnessState>,
}

impl HandoffTokenVecs {
    fn merge(&mut self, other: HandoffTokenVecs) {
        self.provenance_classes.extend(other.provenance_classes);
        self.route_trust_classes.extend(other.route_trust_classes);
        self.capture_permission_states
            .extend(other.capture_permission_states);
        self.redaction_states.extend(other.redaction_states);
        self.continuity_states.extend(other.continuity_states);
        self.boundary_chrome_states
            .extend(other.boundary_chrome_states);
        self.notice_freshness_states
            .extend(other.notice_freshness_states);
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    object_kind: M5HandoffObjectKind,
    qualification: M5HandoffQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    required_fields: &[&str],
    evidence_requirement: M5HandoffEvidenceRequirement,
    required_proof_packet_refs: &[&str],
    downgrade_triggers: Vec<M5HandoffDowngradeTrigger>,
    rollback_posture: M5HandoffRollbackPosture,
    source_contract_refs: &[&str],
    consumer_surfaces: Vec<M5HandoffConsumerSurface>,
) -> M5HandoffObjectRow {
    // Declared vocabularies come straight from the object kind so the row's
    // token vecs and declared list cannot disagree.
    let state_vocabularies = object_kind.required_state_vocabularies().to_vec();
    let mut tokens = HandoffTokenVecs::default();
    for vocab in &state_vocabularies {
        tokens.merge(tokens_for(*vocab));
    }

    M5HandoffObjectRow {
        object_kind,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_fields: strings(required_fields),
        state_vocabularies,
        provenance_classes: tokens.provenance_classes,
        route_trust_classes: tokens.route_trust_classes,
        capture_permission_states: tokens.capture_permission_states,
        redaction_states: tokens.redaction_states,
        continuity_states: tokens.continuity_states,
        boundary_chrome_states: tokens.boundary_chrome_states,
        notice_freshness_states: tokens.notice_freshness_states,
        evidence_requirement,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        downgrade_triggers,
        rollback_posture,
        source_contract_refs: strings(source_contract_refs),
        consumer_surfaces,
    }
}

fn object_rows() -> Vec<M5HandoffObjectRow> {
    use M5HandoffConsumerSurface as S;
    use M5HandoffDowngradeTrigger as D;
    vec![
        row(
            M5HandoffObjectKind::PostInstallNotice,
            M5HandoffQualificationClass::Stable,
            "Help/About owner",
            "Post-install notice / provenance disclosure card that stays inspectable after install; it discloses how the build arrived (official, mirrored, side-loaded, or unknown) and its notice freshness, and never softens an unknown source into an implied official one",
            &[
                "notice_id",
                "provenance_class",
                "notice_freshness_state",
                "inspectable_after_install",
                "disclosure_anchor_ref",
            ],
            M5HandoffEvidenceRequirement::Required,
            &["evidence:post-install-notice-conformance:m5"],
            vec![
                D::NoticeStale,
                D::ProvenanceUnverified,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::NoticeStaysInspectableAfterInstall,
            &[
                M5_HANDOFF_PROVENANCE_BADGE_CONTRACT_REF,
                M5_HANDOFF_PRODUCT_TRUTH_VOCABULARY_REF,
            ],
            vec![
                S::HelpAbout,
                S::UpdateServiceHealth,
                S::SupportExport,
                S::Docs,
                S::ProductUi,
            ],
        ),
        row(
            M5HandoffObjectKind::ProvenanceDisclosure,
            M5HandoffQualificationClass::Stable,
            "Help/About owner",
            "Provenance / source-authenticity disclosure that pins one provenance class and its notice freshness; the marketplace and About surfaces read the same provenance truth, and a degraded or unverified provenance narrows rather than implying authority",
            &[
                "disclosure_id",
                "provenance_class",
                "notice_freshness_state",
                "source_anchor_ref",
                "verification_basis",
            ],
            M5HandoffEvidenceRequirement::Required,
            &["evidence:provenance-disclosure-conformance:m5"],
            vec![
                D::ProvenanceUnverified,
                D::NoticeStale,
                D::UpstreamDependencyNarrowed,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::ProvenanceLabeledNeverImplied,
            &[
                M5_HANDOFF_PROVENANCE_BADGE_CONTRACT_REF,
                M5_HANDOFF_PRODUCT_TRUTH_VOCABULARY_REF,
            ],
            vec![
                S::HelpAbout,
                S::Marketplace,
                S::SupportExport,
                S::Docs,
                S::ProductUi,
            ],
        ),
        row(
            M5HandoffObjectKind::CommunityHandoffRoute,
            M5HandoffQualificationClass::Stable,
            "Ecosystem owner",
            "Official-versus-community outbound route descriptor that declares route trust class, visibility, and support class before launch; a community destination is never presented as an official authenticated one, and a failed or blocked launch retains drafted material and falls back to a local save",
            &[
                "route_id",
                "route_trust_class",
                "continuity_state",
                "visibility_boundary",
                "support_class",
                "fallback_route_ref",
            ],
            M5HandoffEvidenceRequirement::Required,
            &["evidence:community-handoff-route-conformance:m5"],
            vec![
                D::RouteVisibilityUndeclared,
                D::OfflineContinuityLost,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::RouteDeclaresVisibilityBeforeLaunch,
            &[
                M5_HANDOFF_COMMUNITY_PACKET_CONTRACT_REF,
                M5_HANDOFF_TARGET_REVIEW_CONTRACT_REF,
            ],
            vec![
                S::CommunityHandoff,
                S::HelpAbout,
                S::SupportExport,
                S::Docs,
                S::ProductUi,
            ],
        ),
        row(
            M5HandoffObjectKind::ReproductionPacket,
            M5HandoffQualificationClass::Stable,
            "Supportability owner",
            "Redaction-safe reproduction packet that is previewed and redacted before share; raw paths, hostnames, usernames, tokens, and diagnostics are excluded by default, the share is blocked until the preview is confirmed, and a failed handoff keeps the packet retained for a local save",
            &[
                "packet_id",
                "redaction_state",
                "continuity_state",
                "preview_before_share_required",
                "redaction_profile_ref",
            ],
            M5HandoffEvidenceRequirement::Required,
            &[
                "evidence:repro-packet-redaction-conformance:m5",
                "evidence:repro-packet-preview-corpus:m5",
            ],
            vec![
                D::RedactionPreviewMissing,
                D::OfflineContinuityLost,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::RedactionPreviewRequiredBeforeShare,
            &[
                M5_HANDOFF_REPRO_PACKET_CONTRACT_REF,
                M5_HANDOFF_TARGET_REVIEW_CONTRACT_REF,
            ],
            vec![
                S::ReproductionPacket,
                S::CommunityHandoff,
                S::SupportExport,
                S::Docs,
                S::ProductUi,
            ],
        ),
        row(
            M5HandoffObjectKind::OfflineCaptureContinuity,
            M5HandoffQualificationClass::Stable,
            "Supportability owner",
            "Offline-capture continuity record proving capture survives a failed or blocked handoff; the captured material is saved locally with its redaction posture intact and an explicit open-later / retry action, so capture is never lost when a route cannot launch",
            &[
                "capture_id",
                "continuity_state",
                "redaction_state",
                "local_save_anchor_ref",
                "retry_action_ref",
            ],
            M5HandoffEvidenceRequirement::Required,
            &["evidence:offline-capture-continuity-conformance:m5"],
            vec![
                D::OfflineContinuityLost,
                D::RedactionPreviewMissing,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::OfflineCaptureSavedLocal,
            &[
                M5_HANDOFF_REPRO_PACKET_CONTRACT_REF,
                M5_HANDOFF_COMMUNITY_PACKET_CONTRACT_REF,
            ],
            vec![
                S::ReproductionPacket,
                S::CaptureAuthSurface,
                S::SupportExport,
                S::ProductUi,
            ],
        ),
        row(
            M5HandoffObjectKind::DevicePermissionBoundary,
            M5HandoffQualificationClass::Beta,
            "Voice/capture owner",
            "Device / microphone capture permission and capability-limit boundary; the surface states its permission state and stays within the granted capability scope, the capture chrome is clearly disclosed rather than impersonating native chrome, and a revoked or denied permission narrows the claim",
            &[
                "boundary_id",
                "capture_permission_state",
                "boundary_chrome_honesty",
                "granted_capability_scope",
                "permission_anchor_ref",
            ],
            M5HandoffEvidenceRequirement::Required,
            &["evidence:device-permission-boundary-conformance:m5"],
            vec![
                D::CaptureScopeExceeded,
                D::NativeChromeImpersonation,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::CaptureStaysWithinGrantedScope,
            &[
                M5_HANDOFF_SERVICE_HEALTH_CONTRACT_REF,
                M5_HANDOFF_DEPLOYMENT_PROFILES_REF,
            ],
            vec![
                S::CaptureAuthSurface,
                S::HelpAbout,
                S::SupportExport,
                S::Docs,
                S::ProductUi,
            ],
        ),
        row(
            M5HandoffObjectKind::EmbeddedAuthBoundary,
            M5HandoffQualificationClass::Beta,
            "Browser/auth boundary owner",
            "Embedded webview / auth boundary that never impersonates native trusted product chrome; it labels the embedded or external surface and its route trust class so credentials are never entered into a surface posing as native chrome, and an unattributed impersonation is blocked",
            &[
                "boundary_id",
                "boundary_chrome_honesty",
                "route_trust_class",
                "embedded_origin_anchor_ref",
                "credential_entry_posture",
            ],
            M5HandoffEvidenceRequirement::Required,
            &[
                "evidence:embedded-auth-boundary-conformance:m5",
                "evidence:native-chrome-impersonation-rejection-corpus:m5",
            ],
            vec![
                D::NativeChromeImpersonation,
                D::RouteVisibilityUndeclared,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::BoundaryNeverImpersonatesNativeChrome,
            &[
                M5_HANDOFF_COMMUNITY_PACKET_CONTRACT_REF,
                M5_HANDOFF_SERVICE_HEALTH_CONTRACT_REF,
            ],
            vec![
                S::CaptureAuthSurface,
                S::HelpAbout,
                S::SupportExport,
                S::Docs,
                S::ProductUi,
            ],
        ),
        row(
            M5HandoffObjectKind::ServiceHealthNotice,
            M5HandoffQualificationClass::Stable,
            "Service-health owner",
            "Release / service-health communication notice that pins the destination route trust class and its notice freshness; the update and service-health surfaces read the same freshness truth, and a stale or unverified notice narrows rather than implying current service authority",
            &[
                "notice_id",
                "route_trust_class",
                "notice_freshness_state",
                "destination_anchor_ref",
                "freshness_basis",
            ],
            M5HandoffEvidenceRequirement::Required,
            &["evidence:service-health-notice-conformance:m5"],
            vec![
                D::NoticeStale,
                D::RouteVisibilityUndeclared,
                D::UpstreamDependencyNarrowed,
                D::ProofStale,
            ],
            M5HandoffRollbackPosture::RouteDeclaresVisibilityBeforeLaunch,
            &[
                M5_HANDOFF_SERVICE_HEALTH_CONTRACT_REF,
                M5_HANDOFF_PRODUCT_TRUTH_VOCABULARY_REF,
            ],
            vec![
                S::UpdateServiceHealth,
                S::HelpAbout,
                S::ReleaseNotes,
                S::SupportExport,
                S::Docs,
            ],
        ),
    ]
}

fn trust_review() -> M5HandoffTrustReview {
    M5HandoffTrustReview {
        post_install_provenance_inspectable_after_install: true,
        outbound_routes_declare_visibility_and_support_class_before_launch: true,
        repro_packets_previewed_and_redacted_before_share: true,
        offline_capture_survives_failed_handoff: true,
        device_mic_auth_webview_never_impersonates_native_chrome: true,
        provenance_states_distinguish_official_mirrored_side_loaded_unknown: true,
        capture_stays_within_granted_permission_and_capability_limit: true,
        one_handoff_object_model_not_parallel_dialogs: true,
        no_new_community_programs_or_capture_modalities: true,
        redaction_default_excludes_raw_sensitive_material: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5HandoffConsumerProjection {
    M5HandoffConsumerProjection {
        help_about_consumes_handoff_object_model: true,
        marketplace_shows_provenance_class: true,
        update_service_health_shows_route_trust_and_freshness: true,
        community_handoff_declares_visibility_and_support_class: true,
        repro_packets_show_redaction_preview: true,
        capture_auth_surfaces_show_permission_and_chrome_boundary: true,
        support_export_shows_handoff_object_model: true,
        docs_show_provenance_and_redaction_truth: true,
        release_notes_use_controlled_vocabulary: true,
        preview_labs_label_for_unqualified_objects: true,
    }
}

fn proof_freshness() -> M5HandoffProofFreshness {
    M5HandoffProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5HandoffReleasePosture {
    M5HandoffReleasePosture {
        release_packet_ref: "evidence:public-handoff-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:public-handoff-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_REF,
        M5_PUBLIC_HANDOFF_MATRIX_DOC_REF,
        M5_HANDOFF_COMMUNITY_PACKET_CONTRACT_REF,
        M5_HANDOFF_PROVENANCE_BADGE_CONTRACT_REF,
        M5_HANDOFF_SERVICE_HEALTH_CONTRACT_REF,
        M5_HANDOFF_TARGET_REVIEW_CONTRACT_REF,
        M5_HANDOFF_REPRO_PACKET_CONTRACT_REF,
        M5_HANDOFF_PRODUCT_TRUTH_VOCABULARY_REF,
        M5_HANDOFF_DEPLOYMENT_PROFILES_REF,
    ])
}

fn base_input() -> M5PublicHandoffMatrixPacketInput {
    M5PublicHandoffMatrixPacketInput {
        packet_id: M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 Post-Install Notice/Provenance, Community-Handoff, Reproduction-Packet, and Device-Permission/Auth-Boundary Matrix"
                .to_owned(),
        object_rows: object_rows(),
        vocabulary_set: M5HandoffVocabularySet::canonical(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable M5 public-handoff matrix packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_public_handoff_matrix() -> M5PublicHandoffMatrixPacket {
    M5PublicHandoffMatrixPacket::new(base_input())
}

/// Builds a narrowed variant where the reproduction packet is held after a
/// missing-redaction-preview finding, proving downgrade narrows the claim rather
/// than hiding the object.
pub fn seeded_m5_public_handoff_matrix_repro_redaction_held() -> M5PublicHandoffMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-public-handoff-matrix:repro-redaction-held:0001".to_owned();
    for row in &mut input.object_rows {
        if row.object_kind == M5HandoffObjectKind::ReproductionPacket {
            row.qualification = M5HandoffQualificationClass::Held;
            // A held object no longer carries a public claim, so proof becomes
            // recommended rather than required; the object stays visible.
            row.evidence_requirement = M5HandoffEvidenceRequirement::Recommended;
        }
    }
    M5PublicHandoffMatrixPacket::new(input)
}

/// Builds a narrowed variant where the provenance disclosure is pulled to preview
/// after an unverified-provenance finding, proving auto-narrowing keeps the object
/// visible.
pub fn seeded_m5_public_handoff_matrix_provenance_unverified_narrowed(
) -> M5PublicHandoffMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-public-handoff-matrix:provenance-unverified-narrowed:0001".to_owned();
    for row in &mut input.object_rows {
        if row.object_kind == M5HandoffObjectKind::ProvenanceDisclosure {
            row.qualification = M5HandoffQualificationClass::Preview;
        }
    }
    M5PublicHandoffMatrixPacket::new(input)
}
