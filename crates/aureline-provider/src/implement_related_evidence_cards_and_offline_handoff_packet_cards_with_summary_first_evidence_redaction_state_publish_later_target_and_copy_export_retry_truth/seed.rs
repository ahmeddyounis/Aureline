//! Canonical seed builders for the related-evidence / offline-handoff controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical related-evidence / offline-handoff packet.
pub const EVIDENCE_HANDOFF_PACKET_ID: &str =
    "m5-related-evidence-offline-handoff-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn evidence_source_refs() -> Vec<String> {
    strings(&[
        M5_RELATED_EVIDENCE_CARD_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

fn packet_source_refs() -> Vec<String> {
    strings(&[
        M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

/// Builds a related-evidence card, deriving the freshness class and the required notes from
/// the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn evidence_card(
    card_id: &str,
    canonical_id: &str,
    evidence_kind: M5WorkItemEvidenceKind,
    evidence_outcome: EvidenceOutcomeClass,
    summary_label: &str,
    source_label: &str,
    is_reference_current: bool,
    is_freshness_known: bool,
    is_provider_backed: bool,
) -> RelatedEvidenceCard {
    let disclosure = resolve_evidence_card(
        evidence_outcome,
        is_reference_current,
        is_freshness_known,
        is_provider_backed,
    );
    RelatedEvidenceCard {
        component: M5WorkItemComponentFamily::RelatedEvidenceCard,
        card_id: card_id.to_owned(),
        canonical_id: canonical_id.to_owned(),
        evidence_kind,
        evidence_outcome,
        summary_label: summary_label.to_owned(),
        source_label: source_label.to_owned(),
        is_reference_current,
        is_freshness_known,
        is_provider_backed,
        freshness_class: disclosure.freshness_class,
        freshness_note: if disclosure.needs_freshness_note {
            format!(
                "Evidence freshness is {}; open the detail to reconcile before relying on it",
                disclosure.freshness_class.as_str()
            )
        } else {
            String::new()
        },
        failure_note: if disclosure.needs_failure_note {
            "This evidence is failing; open the detail to review the failing checks".to_owned()
        } else {
            String::new()
        },
        leads_with_summary: true,
        actions: EvidenceCardAction::ALL.to_vec(),
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "evidence_kind",
            "outcome",
            "summary",
            "source",
            "freshness",
            "open_detail",
        ]),
        source_contract_refs: evidence_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

/// Builds an offline-handoff packet card, deriving the acceptance class, the acceptance
/// claim, the retry action, and the recovery note from the honest inputs so the seed is
/// always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn packet_card(
    card_id: &str,
    canonical_id: &str,
    packet_type_label: &str,
    handoff_destination: M5WorkItemHandoffDestination,
    publish_later_target_label: &str,
    local_state: M5WorkItemLocalState,
    has_publish_failed: bool,
    included_content_summary: &str,
    export_boundary: M5WorkItemExportBoundary,
    redaction_state_note: &str,
) -> OfflineHandoffPacketCard {
    let disclosure =
        resolve_packet_acceptance(handoff_destination, local_state, has_publish_failed);
    let mut actions = vec![PacketCardAction::CopyPacket, PacketCardAction::ExportPacket];
    if disclosure.needs_retry_action {
        actions.push(PacketCardAction::RetryPublish);
    }
    if disclosure.implies_provider_accepted {
        actions.push(PacketCardAction::OpenInProvider);
    } else {
        actions.push(PacketCardAction::DiscardPacket);
    }
    OfflineHandoffPacketCard {
        component: M5WorkItemComponentFamily::OfflineHandoffPacketCard,
        card_id: card_id.to_owned(),
        canonical_id: canonical_id.to_owned(),
        packet_type_label: packet_type_label.to_owned(),
        handoff_destination,
        publish_later_target_label: publish_later_target_label.to_owned(),
        local_state,
        has_publish_failed,
        acceptance_class: disclosure.acceptance_class,
        implies_provider_accepted: disclosure.implies_provider_accepted,
        included_content_summary: included_content_summary.to_owned(),
        export_boundary,
        redaction_state_note: redaction_state_note.to_owned(),
        failure_recovery_note: if disclosure.needs_failure_recovery_note {
            "The last publish failed; the packet stays here to retry or export".to_owned()
        } else {
            String::new()
        },
        remains_visible_after_failure: true,
        collapses_into_error_banner: false,
        actions,
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "packet_type",
            "included_content",
            "redaction_state",
            "publish_later_target",
            "acceptance",
            "copy_export_retry",
        ]),
        source_contract_refs: packet_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

fn related_evidence_cards() -> Vec<RelatedEvidenceCard> {
    use EvidenceOutcomeClass as Outcome;
    use M5WorkItemEvidenceKind as Kind;

    vec![
        // 1. Failing test, current provider-backed → current evidence, needs failure note.
        evidence_card(
            "evidence-checkout-rounding-tests",
            "PROJ-1421",
            Kind::TestResult,
            Outcome::Failing,
            "3 of 128 checkout tests failing on the rounding path",
            "test run 9042 on branch feature/checkout-rounding",
            true,
            true,
            true,
        ),
        // 2. Passing CI check, current provider-backed → current evidence.
        evidence_card(
            "evidence-checkout-ci",
            "PROJ-1421",
            Kind::CiCheck,
            Outcome::Passing,
            "Lint and build checks passing on the latest push",
            "ci pipeline 5521",
            true,
            true,
            true,
        ),
        // 3. Review thread, informational, out of date → stale evidence.
        evidence_card(
            "evidence-review-thread",
            "PROJ-1421",
            Kind::ReviewThread,
            Outcome::Informational,
            "Review 482 has 2 open threads from an earlier revision",
            "review 482",
            false,
            true,
            true,
        ),
        // 4. Linked change, passing, local-only (not provider-backed) → local-only evidence.
        evidence_card(
            "evidence-local-change",
            "LOCAL-0007",
            Kind::LinkedChange,
            Outcome::Passing,
            "Local worktree change compiles and passes the smoke check",
            "local worktree wt-triage",
            true,
            true,
            false,
        ),
        // 5. Attached artifact, unknown outcome, freshness unknown → unknown freshness.
        evidence_card(
            "evidence-attached-runbook",
            "INC-3390",
            Kind::AttachedArtifact,
            Outcome::UnknownOutcome,
            "Failover runbook attached; last verification unknown",
            "runbook failover-promote",
            true,
            false,
            true,
        ),
        // 6. External reference, informational, current provider-backed → current evidence.
        evidence_card(
            "evidence-adr-reference",
            "EXT-5521",
            Kind::ExternalReference,
            Outcome::Informational,
            "ADR-014 documents the signing-key rotation decision",
            "docs ADR-014",
            true,
            true,
            true,
        ),
    ]
}

fn offline_handoff_packet_cards() -> Vec<OfflineHandoffPacketCard> {
    use M5WorkItemExportBoundary as Boundary;
    use M5WorkItemHandoffDestination as Destination;
    use M5WorkItemLocalState as Local;

    vec![
        // 1. Held in local queue → held-local-only, retryable.
        packet_card(
            "packet-local-draft-hold",
            "LOCAL-0007",
            "Local draft capture",
            Destination::LocalQueue,
            "Local publish-later queue",
            Local::LocalOnlyDraft,
            false,
            "Draft title, state, and linked worktree reference",
            Boundary::MetadataSafe,
            "Metadata-safe: identifiers kept, bodies excluded",
        ),
        // 2. Queued for provider publish → queued-not-yet-accepted, retryable.
        packet_card(
            "packet-queued-comment",
            "PROJ-1421",
            "Queued comment publish",
            Destination::ProviderPublish,
            "acme / checkout board",
            Local::QueuedForPublish,
            false,
            "Pending comment and In-Review transition",
            Boundary::BodyExcluded,
            "Bodies excluded; only the transition metadata is queued",
        ),
        // 3. Prior publish failed → publish-failed-retryable, needs recovery note + retry.
        packet_card(
            "packet-publish-failed",
            "PROJ-1421",
            "Failed transition publish",
            Destination::ProviderPublish,
            "acme / checkout board",
            Local::PublishFailed,
            true,
            "Failed status transition and its linked review reference",
            Boundary::IdentifiersMasked,
            "Identifiers masked; retry keeps the same masked boundary",
        ),
        // 4. Exported to a file → exported-for-handoff.
        packet_card(
            "packet-exported-file",
            "INC-3390",
            "Exported evidence packet",
            Destination::ExportedPacket,
            "Exported handoff file",
            Local::PublishDeferred,
            false,
            "Incident summary, timeline, and attached runbook reference",
            Boundary::CredentialsScrubbed,
            "Credentials scrubbed from the exported packet",
        ),
        // 5. Provider accepted → provider-accepted, not retryable.
        packet_card(
            "packet-provider-accepted",
            "PROJ-1421",
            "Accepted publish receipt",
            Destination::ProviderPublish,
            "acme / checkout board",
            Local::SyncedWithProvider,
            false,
            "Accepted transition receipt and provider confirmation id",
            Boundary::FullDisclosureBlocked,
            "Full disclosure blocked; only the accepted receipt is shown",
        ),
        // 6. Attached to a support bundle → exported-for-handoff.
        packet_card(
            "packet-support-bundle",
            "INC-3390",
            "Support bundle packet",
            Destination::SupportBundle,
            "Support export bundle",
            Local::ConflictHeld,
            false,
            "Conflict-held incident metadata for support review",
            Boundary::LocalOnly,
            "Local-only: nothing leaves the device until you export it",
        ),
        // 7. Handed to another device → exported-for-handoff.
        packet_card(
            "packet-another-device",
            "LOCAL-0007",
            "Device handoff packet",
            Destination::AnotherDevice,
            "Another signed-in device",
            Local::PublishDeferred,
            false,
            "Deferred draft metadata for continuation on another device",
            Boundary::MetadataSafe,
            "Metadata-safe: bodies excluded from the device handoff",
        ),
        // 8. Discarded after review → exported-for-handoff.
        packet_card(
            "packet-discard-review",
            "INC-3390",
            "Discard-after-review packet",
            Destination::DiscardAfterReview,
            "Discard after review",
            Local::ConflictHeld,
            false,
            "Conflict-held change reviewed and marked for discard",
            Boundary::BodyExcluded,
            "Bodies excluded; only the discard decision metadata remains",
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5WorkItemDowngradeTrigger> {
    vec![
        M5WorkItemDowngradeTrigger::IdentityUnstated,
        M5WorkItemDowngradeTrigger::LinkedContextUnstated,
        M5WorkItemDowngradeTrigger::EvidenceProvenanceUnstated,
        M5WorkItemDowngradeTrigger::HandoffDestinationUnstated,
        M5WorkItemDowngradeTrigger::ExportBoundaryHidden,
        M5WorkItemDowngradeTrigger::PublishLaterContinuityHidden,
        M5WorkItemDowngradeTrigger::GenericTicketWordingUsed,
        M5WorkItemDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> EvidenceHandoffTrustReview {
    EvidenceHandoffTrustReview {
        evidence_card_leads_with_summary: true,
        evidence_freshness_derived: true,
        evidence_names_provenance: true,
        evidence_offers_open_detail: true,
        packet_names_type_and_target: true,
        packet_discloses_redaction_state: true,
        offline_packet_never_implies_acceptance: true,
        offline_packet_retryable_after_failure: true,
        offline_packet_exportable_after_failure: true,
        offline_packet_stays_visible_not_error_banner: true,
        no_generic_ticket_wording_conceals_truth: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> EvidenceHandoffConsumerProjection {
    EvidenceHandoffConsumerProjection {
        detail_surface_renders_summary_first_evidence: true,
        offline_surface_keeps_packet_visible_and_retryable: true,
        copy_export_retry_reachable_headless: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> EvidenceHandoffProofFreshness {
    EvidenceHandoffProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        EVIDENCE_HANDOFF_SCHEMA_REF,
        EVIDENCE_HANDOFF_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_RELATED_EVIDENCE_CARD_SCHEMA_REF,
        M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical related-evidence / offline-handoff controls packet.
pub fn seeded_related_evidence_offline_handoff_controls() -> EvidenceHandoffControlsPacket {
    EvidenceHandoffControlsPacket::new(EvidenceHandoffControlsPacketInput {
        packet_id: EVIDENCE_HANDOFF_PACKET_ID.to_owned(),
        surface_label:
            "M5 related-evidence cards and offline-handoff packet cards: evidence cards summarize linked reviews, branches/worktrees, failing/passing tests, CI checks, incidents/runbooks, and docs/ADR references summary-first with derived freshness and an open-detail action; offline-handoff packet cards show packet type, included metadata/evidence, redaction state, and publish-later target, staying visible, retryable, and exportable after failure so a held, queued, or failed packet never implies the provider accepted it"
                .to_owned(),
        related_evidence_cards: related_evidence_cards(),
        offline_handoff_packet_cards: offline_handoff_packet_cards(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights a summary-first related-evidence card whose freshness is
/// derived, so linked evidence leads with a plain summary instead of a raw artifact. Every
/// evidence kind, outcome, and freshness class stays covered so the fixture validates on
/// its own.
pub fn seeded_related_evidence_offline_handoff_controls_related_evidence_summary_first(
) -> EvidenceHandoffControlsPacket {
    let mut packet = seeded_related_evidence_offline_handoff_controls();
    packet.packet_id =
        "m5-related-evidence-offline-handoff-controls:fixture:related-evidence-summary-first"
            .to_owned();
    packet.surface_label =
        "M5 related-evidence cards: linked evidence leads with a summary and an open-detail action, deriving freshness so stale or local-only evidence never reads as current"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights an offline-handoff packet card whose prior publish failed,
/// staying visible, retryable, and exportable rather than collapsing into a generic error
/// banner, and never implying provider acceptance. Every acceptance class, handoff
/// destination, and export boundary stays covered so the fixture validates on its own.
pub fn seeded_related_evidence_offline_handoff_controls_offline_packet_publish_failed(
) -> EvidenceHandoffControlsPacket {
    let mut packet = seeded_related_evidence_offline_handoff_controls();
    packet.packet_id =
        "m5-related-evidence-offline-handoff-controls:fixture:offline-packet-publish-failed"
            .to_owned();
    packet.surface_label =
        "M5 offline-handoff packet cards: a failed packet stays visible, retryable, and exportable, never implying the provider accepted it"
            .to_owned();
    packet
}
