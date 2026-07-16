//! Canonical seed for the constrained-state export and review-evidence packets lane.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV, Markdown
//! summary, health dashboard, and narrowed fixtures. Every binding is derived from one per-entry
//! [`ConstrainedStateGrammar`] so the same seeded entry always carries the same constrained-state grammar across
//! channels, and every binding derives its blocked-write reason, chosen fallback path, write disposition, and
//! checkpoint / undo class from [`resolve_evidence_disclosure`]. The dual-form (human plus machine) evidence, the
//! preserved-versus-lost record, and the redaction record are minted from the same typed decisions so the exported
//! packet stays intelligible without the live UI and never flattens the state class or drops a redaction reason.

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every binding offers so the state class, canonical source, and write target are
/// discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5ConstrainedFileStateAccessibilityRoute> {
    M5ConstrainedFileStateAccessibilityRoute::ALL.to_vec()
}

fn grammar(
    state_role: &str,
    state_class_label: &str,
    blocked_write_reason: &str,
    canonical_source: &str,
    exact_write_target: &str,
    write_disposition: &str,
) -> ConstrainedStateGrammar {
    ConstrainedStateGrammar {
        state_role_word: state_role.to_owned(),
        state_class_label_word: state_class_label.to_owned(),
        blocked_write_reason_word: blocked_write_reason.to_owned(),
        canonical_source_word: canonical_source.to_owned(),
        exact_write_target_word: exact_write_target.to_owned(),
        write_disposition_word: write_disposition.to_owned(),
    }
}

fn canonical_source_join_for(object_class: M5ConstrainedFileStateObject) -> CanonicalSourceJoin {
    CanonicalSourceJoin {
        canonical_source_ref: format!("canonical-source-{}", object_class.as_str()),
        exact_write_target_ref: format!("exact-write-target-{}", object_class.as_str()),
        owning_authority_ref: format!("owning-authority-{}", object_class.as_str()),
        preserved_versus_lost_sync_ref: format!(
            "preserved-versus-lost-sync-{}",
            object_class.as_str()
        ),
    }
}

fn preserved_versus_lost_for(object_class: M5ConstrainedFileStateObject) -> PreservedVersusLost {
    let (retained, lost, path) = match object_class {
        M5ConstrainedFileStateObject::ReadOnly => (
            "The original read-only object stays intact and inspectable",
            "In-place editing of the original path is not available",
            "Edits continue in the duplicated editable copy",
        ),
        M5ConstrainedFileStateObject::Generated => (
            "The generator input and the previous render are kept",
            "Hand edits made directly to the generated artifact are discarded",
            "Regenerate from the canonical source with a preview",
        ),
        M5ConstrainedFileStateObject::PolicyLocked => (
            "The locked object and its approval trail are kept",
            "No edit lands until the policy owner approves",
            "Approval request routes to the policy owner before any write",
        ),
        M5ConstrainedFileStateObject::Managed => (
            "The managed upstream link and its history are kept",
            "Automatic upstream sync stops once the fork is detached",
            "Detach records a restorable checkpoint before local edits",
        ),
        M5ConstrainedFileStateObject::Projection => (
            "The backing source object stays unchanged",
            "Direct edits to the projection surface are not applied in place",
            "Overlay patch layers reversible edits over the backing source",
        ),
        M5ConstrainedFileStateObject::CapturedSnapshot => (
            "The captured snapshot is preserved as a restore point",
            "The snapshot is never mutated in place",
            "Duplicate to an editable copy or restore to the live object",
        ),
    };
    PreservedVersusLost {
        retained: retained.to_owned(),
        lost: lost.to_owned(),
        sync_or_regenerate_path: path.to_owned(),
    }
}

fn state_class_reason_phrase(object_class: M5ConstrainedFileStateObject) -> &'static str {
    match object_class {
        M5ConstrainedFileStateObject::ReadOnly => {
            "the object is reached through a read-only path and an in-place save is blocked"
        }
        M5ConstrainedFileStateObject::Generated => {
            "the artifact is generated from a canonical source and a direct edit is regenerate-only"
        }
        M5ConstrainedFileStateObject::PolicyLocked => {
            "the object is policy-locked and a write is gated behind an approval"
        }
        M5ConstrainedFileStateObject::Managed => {
            "the object is managed by an external owner and a local write requires detaching"
        }
        M5ConstrainedFileStateObject::Projection => {
            "the object is a projection over a backing source and a write requires an overlay patch"
        }
        M5ConstrainedFileStateObject::CapturedSnapshot => {
            "the object is a captured snapshot, not the live object, and is restore-only"
        }
    }
}

fn decision_phrase(decision: ResolvedFallbackDecision) -> &'static str {
    match decision {
        ResolvedFallbackDecision::DuplicatedToEditableCopy => {
            "the operator duplicated it to an editable copy"
        }
        ResolvedFallbackDecision::DetachedFromManagedSource => {
            "the operator detached from the managed source"
        }
        ResolvedFallbackDecision::CreatedOverlayPatch => "the operator created an overlay patch",
        ResolvedFallbackDecision::RequestedApproval => {
            "the operator requested approval from the policy owner"
        }
        ResolvedFallbackDecision::RegeneratedWithPreview => {
            "the operator regenerated it with a preview"
        }
        ResolvedFallbackDecision::Cancelled => {
            "the operator cancelled, leaving the object constrained and unchanged"
        }
    }
}

/// Builds the human-readable line, which always names the specific state-class label and never flattens a non-read-only
/// class into generic read-only language.
fn human_readable_line(
    object_class: M5ConstrainedFileStateObject,
    state_class_label: &str,
    decision: ResolvedFallbackDecision,
) -> String {
    format!(
        "Constrained object classified as {label}: {reason}; {decision}, and the canonical source and exact write target stay named.",
        label = state_class_label,
        reason = state_class_reason_phrase(object_class),
        decision = decision_phrase(decision),
    )
}

fn machine_readable_for(
    object_class: M5ConstrainedFileStateObject,
    disclosure: EvidenceDisclosure,
    decision: ResolvedFallbackDecision,
    join: &CanonicalSourceJoin,
) -> MachineReadableRecord {
    MachineReadableRecord {
        object_class_token: object_class.as_str().to_owned(),
        blocked_write_reason_token: disclosure.blocked_write_reason.as_str().to_owned(),
        canonical_source_ref: join.canonical_source_ref.clone(),
        exact_write_target_ref: join.exact_write_target_ref.clone(),
        chosen_fallback_path_token: disclosure.chosen_fallback_path.as_str().to_owned(),
        resolved_decision_token: decision.as_str().to_owned(),
        write_disposition_token: disclosure.required_write_disposition.as_str().to_owned(),
        checkpoint_undo_class_token: disclosure.checkpoint_undo_class.as_str().to_owned(),
    }
}

fn redaction_record_for(disposition: RedactionDisposition) -> RedactionRecord {
    let omission_reason = match disposition {
        RedactionDisposition::RedactedKeepStateClassAndFallback => Some(
            "Surrounding workspace detail redacted export-safe; state class and fallback decision preserved."
                .to_owned(),
        ),
        RedactionDisposition::NotRedacted => None,
    };
    RedactionRecord {
        disposition,
        omission_reason,
        state_class_preserved: true,
        fallback_decision_preserved: true,
    }
}

fn allowed_actions() -> Vec<EvidenceAction> {
    let mut actions = EvidenceAction::BASE.to_vec();
    actions.push(EvidenceAction::OpenReviewedFallbackReplay);
    actions
}

fn binding_refs(object_class: M5ConstrainedFileStateObject) -> Vec<String> {
    vec![
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    entry_id: &str,
    entry_label: &str,
    object_class: M5ConstrainedFileStateObject,
    channel: EvidencePacketChannel,
    consumer: M5ConstrainedFileStateConsumerSurface,
    decision: ResolvedFallbackDecision,
    redaction_disposition: RedactionDisposition,
    constrained_grammar: ConstrainedStateGrammar,
) -> EvidencePacketBinding {
    let disclosure = resolve_evidence_disclosure(object_class);
    let join = canonical_source_join_for(object_class);
    let machine_readable = machine_readable_for(object_class, disclosure, decision, &join);
    let human_readable_line = human_readable_line(
        object_class,
        &constrained_grammar.state_class_label_word,
        decision,
    );
    EvidencePacketBinding {
        binding_id: binding_id.to_owned(),
        entry_id: entry_id.to_owned(),
        entry_label: entry_label.to_owned(),
        channel,
        consumer,
        object_class,
        co_applicable_object_class: None,
        blocked_write_reason: disclosure.blocked_write_reason,
        chosen_fallback_path: disclosure.chosen_fallback_path,
        resolved_decision: decision,
        write_disposition: disclosure.required_write_disposition,
        checkpoint_undo_class: disclosure.checkpoint_undo_class,
        constrained_grammar,
        dual_form: DualFormEvidence {
            human_readable_line,
            machine_readable,
        },
        preserved_versus_lost: preserved_versus_lost_for(object_class),
        redaction: redaction_record_for(redaction_disposition),
        canonical_source_join: join,
        allowed_actions: allowed_actions(),
        accessibility_routes: all_accessibility_routes(),
        constrained_state_explicitly_classified: true,
        preserves_state_class_and_fallback_when_redacted: true,
        flattens_constrained_state_into_generic_read_only_language: false,
        drops_omission_reason_when_redacted: false,
        lets_one_constrained_state_class_hide_another: false,
        silently_falls_back_to_lossy_direct_write: false,
        gives_ai_automation_import_or_repair_a_hidden_bypass: false,
        leaves_canonical_source_or_exact_write_target_unstated: false,
        presents_as_directly_writable_or_hides_recovery_path: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One channel adoption of a seeded entry, before any override.
struct BindingSpec {
    binding_id: &'static str,
    channel: EvidencePacketChannel,
    consumer: M5ConstrainedFileStateConsumerSurface,
    decision: ResolvedFallbackDecision,
    redaction: RedactionDisposition,
}

/// One seeded constrained-object entry preserved across several packet channels at one constrained-state grammar.
struct EntrySpec {
    entry_id: &'static str,
    entry_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    grammar: ConstrainedStateGrammar,
    bindings: Vec<BindingSpec>,
}

fn entry(
    entry_id: &'static str,
    entry_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    grammar: ConstrainedStateGrammar,
    bindings: Vec<BindingSpec>,
) -> EntrySpec {
    EntrySpec {
        entry_id,
        entry_label,
        object_class,
        grammar,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    channel: EvidencePacketChannel,
    consumer: M5ConstrainedFileStateConsumerSurface,
    decision: ResolvedFallbackDecision,
    redaction: RedactionDisposition,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        channel,
        consumer,
        decision,
        redaction,
    }
}

/// The six seeded entry families — one per constrained-object class — preserved across the four packet channels
/// (support bundle, review / export packet, local-history / restore evidence, docs / help example).
fn entry_specs() -> Vec<EntrySpec> {
    use EvidencePacketChannel::*;
    use M5ConstrainedFileStateConsumerSurface::*;
    use M5ConstrainedFileStateObject::*;
    use RedactionDisposition::*;
    use ResolvedFallbackDecision::*;

    vec![
        entry(
            "read-only-alias-path",
            "Read-only symlink / alias path entry",
            ReadOnly,
            grammar(
                "state_badge_classification",
                "read_only",
                "read_only_path_not_directly_writable",
                "canonical_owning_object",
                "editable_copy_write_target",
                "read_only_blocked",
            ),
            vec![
                bs(
                    "cse-ro-support",
                    SupportBundle,
                    SupportExportPacket,
                    DuplicatedToEditableCopy,
                    NotRedacted,
                ),
                bs(
                    "cse-ro-review",
                    ReviewExportPacket,
                    DiffReviewHeader,
                    DuplicatedToEditableCopy,
                    RedactedKeepStateClassAndFallback,
                ),
                bs(
                    "cse-ro-history",
                    LocalHistoryRestoreEvidence,
                    StatusBar,
                    Cancelled,
                    NotRedacted,
                ),
            ],
        ),
        entry(
            "generated-derived-artifact",
            "Generated / derived artifact entry",
            Generated,
            grammar(
                "blocked_write_reason",
                "generated",
                "generated_artifact_regenerate_only",
                "generator_canonical_source",
                "regenerated_artifact_write_target",
                "regenerate_only",
            ),
            vec![
                bs(
                    "cse-gen-support",
                    SupportBundle,
                    SupportExportPacket,
                    RegeneratedWithPreview,
                    NotRedacted,
                ),
                bs(
                    "cse-gen-review",
                    ReviewExportPacket,
                    WriteReviewSheet,
                    RegeneratedWithPreview,
                    RedactedKeepStateClassAndFallback,
                ),
                bs(
                    "cse-gen-docs",
                    DocsHelpExample,
                    EditorBanner,
                    RegeneratedWithPreview,
                    NotRedacted,
                ),
            ],
        ),
        entry(
            "policy-locked-managed-mirror",
            "Policy-locked managed mirror entry",
            PolicyLocked,
            grammar(
                "canonical_source_relation",
                "policy_locked",
                "policy_lock_requires_approval",
                "policy_owner_authority",
                "approval_gated_write_target",
                "approval_gated",
            ),
            vec![
                bs(
                    "cse-pol-support",
                    SupportBundle,
                    SupportExportPacket,
                    RequestedApproval,
                    NotRedacted,
                ),
                bs(
                    "cse-pol-review",
                    ReviewExportPacket,
                    WriteReviewSheet,
                    RequestedApproval,
                    NotRedacted,
                ),
            ],
        ),
        entry(
            "managed-external-source",
            "Managed, externally-owned source entry",
            Managed,
            grammar(
                "safe_next_step_guidance",
                "managed",
                "managed_source_requires_detach",
                "managing_owner_authority",
                "detached_copy_write_target",
                "detach_required",
            ),
            vec![
                bs(
                    "cse-man-support",
                    SupportBundle,
                    SupportExportPacket,
                    DetachedFromManagedSource,
                    NotRedacted,
                ),
                bs(
                    "cse-man-history",
                    LocalHistoryRestoreEvidence,
                    AiAutomationPath,
                    DetachedFromManagedSource,
                    RedactedKeepStateClassAndFallback,
                ),
                bs(
                    "cse-man-review",
                    ReviewExportPacket,
                    DiffReviewHeader,
                    DetachedFromManagedSource,
                    NotRedacted,
                ),
            ],
        ),
        entry(
            "projection-virtual-view",
            "Projection / virtual view entry",
            Projection,
            grammar(
                "exact_write_target",
                "projection",
                "projection_requires_overlay_or_detach",
                "backing_source_object",
                "overlay_patch_write_target",
                "detach_required",
            ),
            vec![
                bs(
                    "cse-proj-review",
                    ReviewExportPacket,
                    WriteReviewSheet,
                    CreatedOverlayPatch,
                    NotRedacted,
                ),
                bs(
                    "cse-proj-docs",
                    DocsHelpExample,
                    CommandPalette,
                    CreatedOverlayPatch,
                    NotRedacted,
                ),
            ],
        ),
        entry(
            "captured-workspace-snapshot",
            "Captured snapshot in current workspace entry",
            CapturedSnapshot,
            grammar(
                "allowed_blocked_action_set",
                "captured_snapshot",
                "captured_snapshot_restore_only",
                "live_object_source",
                "editable_copy_write_target",
                "read_only_blocked",
            ),
            vec![
                bs(
                    "cse-snap-support",
                    SupportBundle,
                    SupportExportPacket,
                    DuplicatedToEditableCopy,
                    NotRedacted,
                ),
                bs(
                    "cse-snap-history",
                    LocalHistoryRestoreEvidence,
                    TabChrome,
                    Cancelled,
                    NotRedacted,
                ),
                bs(
                    "cse-snap-docs",
                    DocsHelpExample,
                    BreadcrumbTrail,
                    DuplicatedToEditableCopy,
                    NotRedacted,
                ),
            ],
        ),
    ]
}

/// Builds all evidence bindings, applying `override_fn` to override a binding's resolved decision and redaction
/// disposition.
fn build_bindings<F>(override_fn: F) -> Vec<EvidencePacketBinding>
where
    F: Fn(
        &str,
        ResolvedFallbackDecision,
        RedactionDisposition,
    ) -> (ResolvedFallbackDecision, RedactionDisposition),
{
    let mut bindings = Vec::new();
    for entry in entry_specs() {
        for spec in &entry.bindings {
            let (decision, redaction) = override_fn(spec.binding_id, spec.decision, spec.redaction);
            bindings.push(make_binding(
                spec.binding_id,
                entry.entry_id,
                entry.entry_label,
                entry.object_class,
                spec.channel,
                spec.consumer,
                decision,
                redaction,
                entry.grammar.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> EvidenceTrustReview {
    EvidenceTrustReview {
        covers_every_packet_channel: true,
        includes_support_bundle_and_review_export_packet: true,
        every_binding_preserves_both_forms: true,
        machine_readable_mirrors_typed_decision: true,
        no_packet_flattens_into_generic_read_only: true,
        covers_every_resolved_decision_including_cancel: true,
        includes_redacted_binding_keeping_omission_reason: true,
        redacted_bindings_keep_state_class_and_fallback: true,
        constrained_grammar_identical_for_same_entry: true,
        state_role_words_stay_in_frozen_vocabulary: true,
        canonical_source_and_write_target_present_on_every_binding: true,
        every_blocked_write_routes_to_reviewed_fallback: true,
        no_packet_silently_falls_back_to_lossy_direct_write: true,
        no_ai_automation_import_or_repair_bypass: true,
        every_object_class_preserved_by_two_or_more_channels: true,
        accessibility_routes_present_for_state_source_and_target: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> EvidenceConsumerProjection {
    EvidenceConsumerProjection {
        support_bundle_preserves_record: true,
        review_export_packet_preserves_record: true,
        local_history_restore_evidence_preserves_record: true,
        docs_help_example_preserves_record: true,
        every_object_class_preserved_by_two_or_more_channels: true,
        constrained_grammar_identical_for_same_entry: true,
        constrained_state_disclosed_not_flattened: true,
        binding_maps_back_to_one_constrained_object: true,
    }
}

fn proof_freshness() -> EvidenceProofFreshness {
    EvidenceProofFreshness {
        proof_freshness_slo_hours: M5_CONSTRAINED_STATE_EVIDENCE_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_REF.to_owned(),
        M5_CONSTRAINED_STATE_EVIDENCE_DOC_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned(),
    ];
    // The six object classes map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5ConstrainedFileStateObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    evidence_bindings: Vec<EvidencePacketBinding>,
) -> M5ConstrainedStateEvidencePacket {
    M5ConstrainedStateEvidencePacket::new(M5ConstrainedStateEvidencePacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        evidence_bindings,
        downgrade_triggers: ConstrainedStateEvidenceDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5ConstrainedFileStateConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in constrained-state export and review-evidence packets.
pub fn seeded_m5_constrained_state_evidence_packets() -> M5ConstrainedStateEvidencePacket {
    packet_from_bindings(
        M5_CONSTRAINED_STATE_EVIDENCE_PACKET_ID,
        "M5 constrained-state export and review-evidence packets (preserved class, source, write target, and fallback)",
        build_bindings(|_, decision, redaction| (decision, redaction)),
    )
}

/// Fixture: two additional support / review packets narrowed to a redacted disposition, proving that redaction-aware
/// export keeps the omission reason and still preserves the state class and fallback decision on more packets. The
/// non-redacted disposition stays covered on the other bindings.
pub fn seeded_m5_constrained_state_evidence_packets_redaction_narrowed(
) -> M5ConstrainedStateEvidencePacket {
    packet_from_bindings(
        "m5-constrained-state-evidence:redaction:0001",
        "M5 constrained-state evidence packets (redaction narrowed)",
        build_bindings(|binding_id, decision, redaction| match binding_id {
            "cse-pol-support" | "cse-snap-support" => (
                decision,
                RedactionDisposition::RedactedKeepStateClassAndFallback,
            ),
            _ => (decision, redaction),
        }),
    )
}

/// Fixture: two resolved decisions narrowed to a cancellation, proving the packet preserves a cancelled decision (the
/// object stayed constrained and unchanged) just as faithfully as a taken fallback. The regenerate and detach
/// decisions stay covered on their other bindings.
pub fn seeded_m5_constrained_state_evidence_packets_cancelled_decision_narrowed(
) -> M5ConstrainedStateEvidencePacket {
    packet_from_bindings(
        "m5-constrained-state-evidence:cancelled:0001",
        "M5 constrained-state evidence packets (cancelled decision narrowed)",
        build_bindings(|binding_id, decision, redaction| match binding_id {
            "cse-gen-support" | "cse-man-support" => {
                (ResolvedFallbackDecision::Cancelled, redaction)
            }
            _ => (decision, redaction),
        }),
    )
}
