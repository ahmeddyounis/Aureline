//! Canonical seed builders for the M5 review-template-packet and template-publish-attribution registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean review-template-packet and
//! template-publish-attribution entries are built so the one typed comment / summary template packet
//! per review — its pack-authored rationale blocks, checklist text, and bundle manifests bound to the
//! template version and pack digest — user-edited and redacted fields never read as pack-authored template
//! content, a draft that cannot name its template version / pack digest degrades honestly rather than
//! flattening into generic review text, the canonical / accessible / audit resolution forms, and the
//! complete field-provenance / template-version-and-digest / destination-and-redaction attribution object
//! are proven across the review-detail, AI-review, review-pack-summary, local-CI-parity, provider-handoff,
//! and support surfaces without any hand-copied per-entry assumption, template version / pack digest applied
//! silently, incomplete object, undisclosed user-edit or redaction, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_PACKET_ID: &str =
    "m5-review-template-packet-and-publish-attribution-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_template_packet(
    input: M5ReviewTemplatePacketEntryResolutionInput,
) -> M5ResolvedReviewTemplatePacketEntry {
    resolve_review_template_packet_entry(input)
        .expect("seed line-review_template_packet entry resolves")
}

fn downgrade(
    input: M5TemplatePublishAttributionEntryResolutionInput,
) -> M5ResolvedTemplatePublishAttributionEntry {
    resolve_template_publish_attribution_entry(input)
        .expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5ReviewTemplatePacketResolutionForm> {
    M5ReviewTemplatePacketResolutionForm::ALL.to_vec()
}

// -- Clean line-review_template_packet entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_template_packet_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    report_section: M5ReviewTemplatePacketKind,
    surface_context: M5ReviewTemplatePacketSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5ReviewTemplatePacketEntryResolutionInput {
    M5ReviewTemplatePacketEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        line_binding_id: line_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        report_section,
        surface_context,
        resolution_form_coverage: all_forms(),
        exact_repo_journey_rows: exact_repo_journey_rows.to_owned(),
        bundle_ids: bundle_ids.to_owned(),
        install_topology: install_topology.to_owned(),
        toolchain_envelope: toolchain_envelope.to_owned(),
        known_limits: known_limits.to_owned(),
        rollback_target: rollback_target.to_owned(),
        diagnostics_posture: diagnostics_posture.to_owned(),
        bound_to_registry: true,
        rollback_and_diagnostics_bounded: true,
        is_public_facing_line: false,
        support_language_matches_line_proof: true,
        proof_fresh: true,
    }
}

fn review_template_packet_rationale_block_clean() -> M5ResolvedReviewTemplatePacketEntry {
    review_template_packet(clean_review_template_packet_base(
        "review_template_packet:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_template_packet.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewTemplatePacketKind::RationaleBlock,
        M5ReviewTemplatePacketSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_template_packet_checklist_text_clean() -> M5ResolvedReviewTemplatePacketEntry {
    review_template_packet(clean_review_template_packet_base(
        "review_template_packet:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_template_packet.checklist_text",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewTemplatePacketKind::ChecklistText,
        M5ReviewTemplatePacketSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_template_packet_bundle_manifest_clean() -> M5ResolvedReviewTemplatePacketEntry {
    review_template_packet(clean_review_template_packet_base(
        "review_template_packet:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_template_packet.bundle_manifest",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewTemplatePacketKind::BundleManifest,
        M5ReviewTemplatePacketSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_template_packet_user_edited_field_clean() -> M5ResolvedReviewTemplatePacketEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_template_packet_base(
        "review_template_packet:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_template_packet.user_edited_field",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewTemplatePacketKind::UserEditedField,
        M5ReviewTemplatePacketSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-journeys",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.is_public_facing_line = true;
    base.support_language_matches_line_proof = true;
    review_template_packet(base)
}

fn review_template_packet_redacted_field_clean() -> M5ResolvedReviewTemplatePacketEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_template_packet_base(
        "review_template_packet:support:public-preview",
        "launch.line.public-preview",
        "line.review_template_packet.redacted_field",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewTemplatePacketKind::RedactedField,
        M5ReviewTemplatePacketSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-journeys",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.is_public_facing_line = true;
    base.support_language_matches_line_proof = true;
    review_template_packet(base)
}

fn review_template_packet_summary_text_clean() -> M5ResolvedReviewTemplatePacketEntry {
    review_template_packet(clean_review_template_packet_base(
        "review_template_packet:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_template_packet.summary_text",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewTemplatePacketKind::SummaryText,
        M5ReviewTemplatePacketSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-review_template_packet entries ---------------------------------------------------------

/// Degraded review_template_packet entry: the resolved review_template_packet object is incomplete — the bundle IDs are unstated.
fn review_template_packet_object_incomplete() -> M5ResolvedReviewTemplatePacketEntry {
    let mut base = clean_review_template_packet_base(
        "review_template_packet:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_template_packet.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewTemplatePacketKind::RationaleBlock,
        M5ReviewTemplatePacketSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    review_template_packet(base)
}

/// Degraded review_template_packet entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn review_template_packet_widen_fold() -> M5ResolvedReviewTemplatePacketEntry {
    let mut base = clean_review_template_packet_base(
        "review_template_packet:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_template_packet.checklist_text",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewTemplatePacketKind::ChecklistText,
        M5ReviewTemplatePacketSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    review_template_packet(base)
}

/// Degraded review_template_packet entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn review_template_packet_unbound() -> M5ResolvedReviewTemplatePacketEntry {
    let mut base = clean_review_template_packet_base(
        "review_template_packet:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_template_packet.summary_text",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewTemplatePacketKind::SummaryText,
        M5ReviewTemplatePacketSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    review_template_packet(base)
}

/// Degraded review_template_packet entry: the canonical registry token name is unstated.
fn review_template_packet_token_unstated() -> M5ResolvedReviewTemplatePacketEntry {
    let mut base = clean_review_template_packet_base(
        "review_template_packet:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewTemplatePacketKind::BundleManifest,
        M5ReviewTemplatePacketSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    review_template_packet(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    comparison_scope: M5TemplatePublishAttributionScope,
    surface_context: M5ReviewTemplatePacketSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5TemplatePublishAttributionEntryResolutionInput {
    M5TemplatePublishAttributionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        comparison_ref: comparison_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        comparison_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_line_identity: resolved_line_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_template_publish_attribution_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedTemplatePublishAttributionEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5TemplatePublishAttributionScope::FieldProvenanceBinding,
        M5ReviewTemplatePacketSurfaceContext::ShiproomSurface,
        "line-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = true;
    downgrade(base)
}

fn downgrade_rehearsal_currency_clean() -> M5ResolvedTemplatePublishAttributionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.bundle_manifest",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5TemplatePublishAttributionScope::TemplateVersionAndDigestBinding,
        M5ReviewTemplatePacketSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedTemplatePublishAttributionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.summary_text",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5TemplatePublishAttributionScope::DestinationAndRedactionBinding,
        M5ReviewTemplatePacketSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded line-downgrade-packet entries ----------------------------------------------------

/// Degraded downgrade entry: the downgrade would run partner / public support language ahead of line proof — a
/// support-language reference present but not bound to line proof reads as trustworthy when the line proof
/// does not yet back it.
fn downgrade_support_ahead() -> M5ResolvedTemplatePublishAttributionEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.redacted_field",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5TemplatePublishAttributionScope::FieldProvenanceBinding,
        M5ReviewTemplatePacketSurfaceContext::ShiproomSurface,
        "line-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = false;
    downgrade(base)
}

/// Degraded downgrade entry: the canonical / accessible / audit resolution-form coverage of the downgrade is
/// incomplete.
fn downgrade_form_incomplete() -> M5ResolvedTemplatePublishAttributionEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.summary_text",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5TemplatePublishAttributionScope::DestinationAndRedactionBinding,
        M5ReviewTemplatePacketSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5ReviewTemplatePacketResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_template_publish_attribution_unclassified(
) -> M5ResolvedTemplatePublishAttributionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.user_edited_field",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5TemplatePublishAttributionScope::TemplatePublishAttributionUnclassified,
        M5ReviewTemplatePacketSurfaceContext::ExecutiveSteeringSurface,
        "line-id.design-partner-preview-0007",
        "known-limits.ledger.design-partner",
        "rollback.target.ref.design-partner",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.design-partner-reviewed",
        "support.language.design-partner-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ReviewTemplatePacketAndPublishAttributionRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    review_template_packet_entries: Vec<M5ResolvedReviewTemplatePacketEntry>,
    template_publish_attribution_entries: Vec<M5ResolvedTemplatePublishAttributionEntry>,
) -> M5ReviewTemplatePacketAndPublishAttributionRegistriesRow {
    M5ReviewTemplatePacketAndPublishAttributionRegistriesRow {
        consumer_surface,
        qualification: M5ReviewPackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ReviewPackClassificationStage::ALL.to_vec(),
        required_labels: M5ReviewPackRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ReviewPackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ReviewTemplatePacketAnatomyPart::ALL.to_vec(),
        export_fields: M5ReviewTemplatePacketExportField::ALL.to_vec(),
        downgrade_triggers,
        review_template_packet_entries,
        template_publish_attribution_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_SCHEMA_REF,
            M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF,
            M5_TEMPLATE_PUBLISH_ATTRIBUTION_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_template_publish_attribution_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5ReviewTemplatePacketAndPublishAttributionRegistriesRow> {
    use M5ReviewPackConsumerSurface as C;
    use M5ReviewPackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves the active review pack to one typed comment / summary template packet — the pack-authored rationale blocks, the checklist text, the bundle manifests, the template version, and the pack digest — bound to the same review-pack version and content digest as human, local, and CI review, and proves the publish-attribution binding for the draft (which fields are pack-authored, generated, user-edited, omitted, or redacted); a packet that cannot name the template version and pack digest it is bound to and an attribution that would let user-edited or redacted text read as pack-authored template content degrade honestly instead of flattening the template into generic review text across local draft, publish-now, open-in-provider, and export",
            "review-pack:m5-review-detail:001",
            vec![
                D::PackVersionDigestUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_template_packet_rationale_block_clean(),
                review_template_packet_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review owner",
            "The AI review panel resolves the template-version-and-digest binding and the field-provenance attribution while keeping the bound template version / pack digest and whether each field is pack-authored, generated, user-edited, omitted, or redacted visible; a packet operating with a template version / pack digest that cannot be named and a resolution-form gap on an attribution are caught before a green summary can present the draft as clean pack-authored content, and AI review can never publish template-driven content under a different or undisclosed template version",
            "review-pack:m5-ai-review:001",
            vec![
                D::PackVersionDigestUnstated,
                D::PackVersionOrDigestDropped,
                D::ReviewPackMatrixStale,
            ],
            vec![review_template_packet_checklist_text_clean(), review_template_packet_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the field-provenance class while keeping the template version / pack digest and the template attribution bound to the export, and reports the destination-and-redaction state; a packet that is a hand-copied per-entry assumption and an attribution on an unclassified binding degrade honestly so the template version, pack digest, and authorship provenance are never dropped on export or reopen",
            "review-pack:m5-support:001",
            vec![
                D::ParityStateUnstated,
                D::PackFreshnessUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_template_packet_user_edited_field_clean(),
                review_template_packet_unbound(),
            ],
            vec![comparison_template_publish_attribution_unclassified()],
        ),
        base_row(
            C::ReviewPackSummary,
            "Review-pack-summary owner",
            "The review-pack summary resolves the bundle manifests and checklist text and the destination-and-redaction state — destination, template source, and redaction state shown — bound to the registry so template-driven review content can never be sent without first showing where it goes, which template authored it, and what was redacted; an unstated template version / pack digest on a packet is caught before it can drift",
            "review-pack:m5-review-pack-summary:001",
            vec![
                D::ParityStateUnstated,
                D::EvaluatorResultClassUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_template_packet_bundle_manifest_clean(),
                review_template_packet_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::LocalCiParityStrip,
            "Local-CI-parity owner",
            "The local-CI parity strip renders the same resolved template-packet and publish-attribution truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the user-edited / redacted field labels and the field-provenance binding stay inspectable off-renderer so user edits and redactions never read as pack-authored template content",
            "review-pack:m5-local-ci-parity:001",
            vec![
                D::EvaluatorResultClassUnstated,
                D::ParityStateUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![review_template_packet_summary_text_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderHandoff,
            "Provider-handoff owner",
            "The provider handoff feed carries the same resolved template-packet and publish-attribution truth into browser / provider handoff and reopened draft-only review state, so a dropped template version / pack digest, undisclosed template attribution, user-edited or redacted text shown as pack-authored, or a send without destination and redaction state is visible in evidence — a field-provenance change, a template-version-and-digest change, or a destination-and-redaction change — rather than hidden behind a green summary",
            "review-pack:m5-provider-handoff:001",
            vec![
                D::ParityStateUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![review_template_packet_redacted_field_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5ReviewTemplatePacketAndPublishAttributionRegistriesGovernanceReview {
    M5ReviewTemplatePacketAndPublishAttributionRegistriesGovernanceReview {
        review_template_packet_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_review_template_packet_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        template_publish_attribution_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        review_template_packet_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ReviewTemplatePacketAndPublishAttributionRegistriesConsumerProjection
{
    M5ReviewTemplatePacketAndPublishAttributionRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ReviewTemplatePacketAndPublishAttributionRegistriesProofFreshness {
    M5ReviewTemplatePacketAndPublishAttributionRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReviewTemplatePacketAndPublishAttributionRegistriesReleasePosture {
    M5ReviewTemplatePacketAndPublishAttributionRegistriesReleasePosture {
        proof_packet_ref: M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_SCHEMA_REF,
        M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF,
        M5_TEMPLATE_PUBLISH_ATTRIBUTION_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 review-template-packet and template-publish-attribution registries packet.
pub fn seeded_m5_review_template_packet_and_publish_attribution_registries(
) -> M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket {
    M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket::new(
        M5ReviewTemplatePacketAndPublishAttributionRegistriesPacketInput {
            packet_id: M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 comment / summary review-template-packet and template-publish-attribution registries binding one typed template packet per review to the active review pack — the pack-authored rationale blocks, the checklist text, the bundle manifests, the template version, and the pack digest, each bound to the same review-pack version / content digest as human, local, and CI review — so published, draft, and exported review summaries preserve template version and pack digest instead of flattening them into generic review text across local draft, publish-now, open-in-provider, and export, with canonical / accessible / audit resolution-form coverage, and a machine-readable template-publish-attribution (field-provenance-binding, template-version-and-digest-binding, or destination-and-redaction-binding) that surfaces whether each field is pack-authored, generated, user-edited, omitted, or redacted, keeps user edits and redactions visibly separate from pack-authored template content, and never sends template-driven review content without first showing destination, template source, and redaction state across review, AI-review, provider-handoff, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5ReviewTemplatePacketAndPublishAttributionRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the review-detail row is held at Beta pending review-template-packet parity on every pack
/// version / digest; every row stays visible and every example stays honest.
pub fn seeded_m5_review_template_packet_and_publish_attribution_registries_review_template_packet_beta_narrowed(
) -> M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.packet_id =
        "m5-review-template-packet-and-publish-attribution-registries:review-template-packet-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::ReviewDetail)
        .expect("review-detail row present");
    row.qualification = M5ReviewPackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending template-publish-attribution parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_review_template_packet_and_publish_attribution_registries_template_publish_attribution_preview_narrowed(
) -> M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.packet_id =
        "m5-review-template-packet-and-publish-attribution-registries:template-publish-attribution-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::AiReviewPanel)
        .expect("AI-review row present");
    row.qualification = M5ReviewPackQualificationClass::Preview;
    packet
}
