//! Canonical seed builders for the M5 diagnostic-decoration / code-action-chip controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean diagnostic
//! decorations and code-action chips are built so the shared severity/source/freshness and
//! fix-posture/apply-scope grammar is proven across editor, diagnostics, notebook, AI, support, and
//! product surfaces without any color-only encoding, silent anchor drift, overstated imported
//! certainty, inferred-as-exact fix, or bypassed preview.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_PACKET_ID: &str =
    "m5-diagnostic-decoration-code-action-chip-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn decoration(input: M5DiagnosticDecorationResolutionInput) -> M5ResolvedDiagnosticDecoration {
    resolve_diagnostic_decoration(input).expect("seed diagnostic decoration input resolves")
}

fn chip(input: M5CodeActionChipResolutionInput) -> M5ResolvedCodeActionChip {
    resolve_code_action_chip(input).expect("seed code-action chip input resolves")
}

// -- Clean diagnostic-decoration examples (shared severity/source/freshness grammar) ------------

#[allow(clippy::too_many_arguments)]
fn clean_decoration_base(
    decoration_id: &str,
    message: &str,
    severity: M5DiagnosticSeverity,
    source: M5DiagnosticSourceClass,
    freshness: M5DiagnosticFreshness,
    anchor: M5AnchorDurability,
    linkage: M5DiagnosticLinkageTarget,
) -> M5DiagnosticDecorationResolutionInput {
    M5DiagnosticDecorationResolutionInput {
        decoration_id: decoration_id.to_owned(),
        message_label: message.to_owned(),
        severity,
        severity_stated: true,
        source_class: source,
        freshness,
        stale_disclosed: true,
        anchor_durability: anchor,
        anchor_drift_disclosed: true,
        linkage_target: linkage,
        linkage_stable: true,
        imported_certainty_distinguished: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean error diagnostic from a language server, anchored exactly, linked to Problems.
fn dec_error_ls_clean() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:editor:error-77",
        "mismatched types: expected `u32`, found `String`",
        M5DiagnosticSeverity::Error,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    ))
}

/// Clean warning diagnostic from a linter.
fn dec_warning_linter_clean() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:editor:warning-31",
        "unused import `std::fmt`",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::Linter,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    ))
}

/// Clean info diagnostic from the compiler, cleanly re-anchored, linked to an output channel.
fn dec_info_compiler_clean() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:diagnostics:info-12",
        "note: consider borrowing here",
        M5DiagnosticSeverity::Info,
        M5DiagnosticSourceClass::Compiler,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::ReAnchored,
        M5DiagnosticLinkageTarget::OutputChannel,
    ))
}

/// Clean hint diagnostic from a test runner, linked to the diagnostics export.
fn dec_hint_testrunner_clean() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:diagnostics:hint-4",
        "hint: assertion could use `assert_eq!`",
        M5DiagnosticSeverity::Hint,
        M5DiagnosticSourceClass::TestRunner,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::DiagnosticsExport,
    ))
}

/// Clean stale diagnostic that discloses its staleness rather than reading as current.
fn dec_stale_disclosed_clean() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:notebook:stale-9",
        "warning: value assigned is never read (recomputing)",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Stale,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    ))
}

/// Clean imported / external diagnostic whose certainty is distinguished from a native run.
fn dec_imported_distinguished_clean() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:support:imported-3",
        "imported: external analyzer flagged possible null deref",
        M5DiagnosticSeverity::Error,
        M5DiagnosticSourceClass::ImportedExternal,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::SupportPacket,
    ))
}

/// Clean warning diagnostic, cleanly re-anchored, surfaced inline in an AI context.
fn dec_reanchored_ai_clean() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:ai:reanchored-18",
        "warning: this match arm is unreachable",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::ReAnchored,
        M5DiagnosticLinkageTarget::EditorInline,
    ))
}

// -- Degraded diagnostic-decoration examples ---------------------------------------------------

/// Degraded decoration: the problem identity / message is unstated.
fn dec_identity_unstated() -> M5ResolvedDiagnosticDecoration {
    let mut input = clean_decoration_base(
        "decoration:support:no-message",
        "   ",
        M5DiagnosticSeverity::Error,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    );
    input.message_label = "   ".to_owned();
    decoration(input)
}

/// Degraded decoration: the severity cannot be resolved.
fn dec_severity_unresolved() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:diagnostics:severity-unknown",
        "diagnostic with no resolvable severity",
        M5DiagnosticSeverity::SeverityUnknown,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    ))
}

/// Degraded decoration: the severity is encoded by color alone.
fn dec_severity_color_only() -> M5ResolvedDiagnosticDecoration {
    let mut input = clean_decoration_base(
        "decoration:editor:severity-color-only",
        "problem shown only by a red underline",
        M5DiagnosticSeverity::Error,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    );
    input.severity_stated = false;
    decoration(input)
}

/// Degraded decoration: the source / provider is unstated.
fn dec_source_unstated() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:product:source-unknown",
        "diagnostic with no resolvable source",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::SourceUnknown,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    ))
}

/// Degraded decoration: the freshness cannot be resolved.
fn dec_freshness_unresolved() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:diagnostics:freshness-unknown",
        "diagnostic with no resolvable freshness",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::FreshnessUnknown,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    ))
}

/// Degraded decoration: a stale diagnostic is shown as current.
fn dec_stale_shown() -> M5ResolvedDiagnosticDecoration {
    let mut input = clean_decoration_base(
        "decoration:diagnostics:stale-shown",
        "warning that is actually stale but presented as current",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Stale,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    );
    input.stale_disclosed = false;
    decoration(input)
}

/// Degraded decoration: the anchor durability cannot be resolved.
fn dec_anchor_unresolved() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:product:anchor-unknown",
        "diagnostic with no resolvable anchor",
        M5DiagnosticSeverity::Error,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchorUnresolved,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    ))
}

/// Degraded decoration: the anchor drifted / went outdated without being disclosed.
fn dec_anchor_drift_hidden() -> M5ResolvedDiagnosticDecoration {
    let mut input = clean_decoration_base(
        "decoration:notebook:anchor-drift",
        "diagnostic whose anchor silently drifted",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::OutdatedAnchor,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    );
    input.anchor_drift_disclosed = false;
    decoration(input)
}

/// Degraded decoration: the linkage target cannot be resolved.
fn dec_linkage_unresolved() -> M5ResolvedDiagnosticDecoration {
    decoration(clean_decoration_base(
        "decoration:product:linkage-unknown",
        "diagnostic with no resolvable linkage target",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::LinkageUnresolved,
    ))
}

/// Degraded decoration: the linkage to Problems / output / support is broken.
fn dec_linkage_broken() -> M5ResolvedDiagnosticDecoration {
    let mut input = clean_decoration_base(
        "decoration:support:linkage-broken",
        "diagnostic whose Problems linkage is broken",
        M5DiagnosticSeverity::Error,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    );
    input.linkage_stable = false;
    decoration(input)
}

/// Degraded decoration: an imported diagnostic overstates its certainty.
fn dec_imported_overstated() -> M5ResolvedDiagnosticDecoration {
    let mut input = clean_decoration_base(
        "decoration:ai:imported-overstated",
        "imported diagnostic presented with native certainty",
        M5DiagnosticSeverity::Error,
        M5DiagnosticSourceClass::ImportedExternal,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::SupportPacket,
    );
    input.imported_certainty_distinguished = false;
    decoration(input)
}

/// Degraded decoration: no command-backed detail path is reachable.
fn dec_detail_missing() -> M5ResolvedDiagnosticDecoration {
    let mut input = clean_decoration_base(
        "decoration:product:detail-missing",
        "diagnostic with no command-backed detail path",
        M5DiagnosticSeverity::Warning,
        M5DiagnosticSourceClass::LanguageServer,
        M5DiagnosticFreshness::Current,
        M5AnchorDurability::AnchoredExact,
        M5DiagnosticLinkageTarget::ProblemsPanel,
    );
    input.detail_command_available = false;
    decoration(input)
}

// -- Clean code-action-chip examples -----------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_chip_base(
    chip_id: &str,
    label: &str,
    posture: M5FixPosture,
    scope: M5CodeActionApplyScope,
    side_effect: M5CodeActionSideEffectClass,
    block_reason: M5CodeActionBlockReason,
) -> M5CodeActionChipResolutionInput {
    M5CodeActionChipResolutionInput {
        chip_id: chip_id.to_owned(),
        action_label: label.to_owned(),
        fix_posture: posture,
        posture_stated: true,
        shown_as_exact: matches!(posture, M5FixPosture::ExactFix),
        apply_scope: scope,
        preview_available: true,
        side_effect_class: side_effect,
        side_effect_disclosed: true,
        block_reason,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean exact fix applied directly, single-file.
fn chip_exact_direct_clean() -> M5ResolvedCodeActionChip {
    clean_chip(clean_chip_base(
        "chip:editor:exact-import",
        "Add missing import `std::fmt`",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::DirectApply,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    ))
}

/// Clean inferred fix that requires a preview and discloses its multi-file side effect.
fn chip_inferred_preview_clean() -> M5ResolvedCodeActionChip {
    clean_chip(clean_chip_base(
        "chip:editor:inferred-rename",
        "Rename symbol across module (inferred)",
        M5FixPosture::InferredFix,
        M5CodeActionApplyScope::PreviewRequired,
        M5CodeActionSideEffectClass::MultiFile,
        M5CodeActionBlockReason::NotBlocked,
    ))
}

/// Clean heuristic suggestion routed through review, workspace-wide, side effect disclosed.
fn chip_heuristic_review_clean() -> M5ResolvedCodeActionChip {
    clean_chip(clean_chip_base(
        "chip:diagnostics:heuristic-format",
        "Reformat workspace (heuristic suggestion)",
        M5FixPosture::HeuristicSuggestion,
        M5CodeActionApplyScope::ReviewRequired,
        M5CodeActionSideEffectClass::WorkspaceWide,
        M5CodeActionBlockReason::NotBlocked,
    ))
}

/// Clean blocked action that honestly names its policy-denied block reason.
fn chip_blocked_clean() -> M5ResolvedCodeActionChip {
    clean_chip(clean_chip_base(
        "chip:ai:blocked-policy",
        "Apply generated patch (blocked)",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::Blocked,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::PolicyDenied,
    ))
}

/// Clean exact fix touching external state, previewed and disclosed.
fn chip_external_disclosed_clean() -> M5ResolvedCodeActionChip {
    clean_chip(clean_chip_base(
        "chip:ai:external-migrate",
        "Run data migration (touches external state)",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::PreviewRequired,
        M5CodeActionSideEffectClass::ExternalState,
        M5CodeActionBlockReason::NotBlocked,
    ))
}

fn clean_chip(input: M5CodeActionChipResolutionInput) -> M5ResolvedCodeActionChip {
    chip(input)
}

// -- Degraded code-action-chip examples --------------------------------------------------------

/// Degraded chip: the chip identity / action label is unstated.
fn chip_identity_unstated() -> M5ResolvedCodeActionChip {
    let mut input = clean_chip_base(
        "chip:support:no-label",
        "  ",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::DirectApply,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    );
    input.action_label = "  ".to_owned();
    chip(input)
}

/// Degraded chip: the fix posture cannot be resolved.
fn chip_posture_unresolved() -> M5ResolvedCodeActionChip {
    chip(clean_chip_base(
        "chip:diagnostics:posture-unknown",
        "Quick fix with unresolved posture",
        M5FixPosture::PostureUnknown,
        M5CodeActionApplyScope::DirectApply,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    ))
}

/// Degraded chip: the fix posture is encoded by color alone.
fn chip_posture_color_only() -> M5ResolvedCodeActionChip {
    let mut input = clean_chip_base(
        "chip:editor:posture-color-only",
        "Fix shown only by chip color",
        M5FixPosture::InferredFix,
        M5CodeActionApplyScope::PreviewRequired,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    );
    input.posture_stated = false;
    chip(input)
}

/// Degraded chip: an inferred fix is presented as exact.
fn chip_inferred_as_exact() -> M5ResolvedCodeActionChip {
    let mut input = clean_chip_base(
        "chip:ai:inferred-as-exact",
        "Fix imports (presented as exact)",
        M5FixPosture::InferredFix,
        M5CodeActionApplyScope::DirectApply,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    );
    input.shown_as_exact = true;
    chip(input)
}

/// Degraded chip: the apply scope cannot be resolved.
fn chip_scope_unresolved() -> M5ResolvedCodeActionChip {
    chip(clean_chip_base(
        "chip:diagnostics:scope-unknown",
        "Fix with unresolved apply scope",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::ScopeUnresolved,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    ))
}

/// Degraded chip: a preview-required action bypasses its preview.
fn chip_preview_bypassed() -> M5ResolvedCodeActionChip {
    let mut input = clean_chip_base(
        "chip:support:preview-bypassed",
        "Apply fix without the required preview",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::PreviewRequired,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    );
    input.preview_available = false;
    chip(input)
}

/// Degraded chip: a blocked action hides its reason.
fn chip_blocked_reason_hidden() -> M5ResolvedCodeActionChip {
    chip(clean_chip_base(
        "chip:support:blocked-reason-hidden",
        "Apply fix (blocked, no reason shown)",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::Blocked,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::BlockReasonUnknown,
    ))
}

/// Degraded chip: the side-effect class cannot be resolved.
fn chip_side_effect_unresolved() -> M5ResolvedCodeActionChip {
    chip(clean_chip_base(
        "chip:product:side-effect-unknown",
        "Fix with unresolved side-effect class",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::DirectApply,
        M5CodeActionSideEffectClass::SideEffectUnknown,
        M5CodeActionBlockReason::NotBlocked,
    ))
}

/// Degraded chip: a multi-file fix hides its side-effect class.
fn chip_side_effect_hidden() -> M5ResolvedCodeActionChip {
    let mut input = clean_chip_base(
        "chip:notebook:side-effect-hidden",
        "Apply multi-file fix (side effect hidden)",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::PreviewRequired,
        M5CodeActionSideEffectClass::MultiFile,
        M5CodeActionBlockReason::NotBlocked,
    );
    input.side_effect_disclosed = false;
    chip(input)
}

/// Degraded chip: no command-backed detail path is reachable.
fn chip_detail_missing() -> M5ResolvedCodeActionChip {
    let mut input = clean_chip_base(
        "chip:product:detail-missing",
        "Apply fix with no command-backed detail path",
        M5FixPosture::ExactFix,
        M5CodeActionApplyScope::DirectApply,
        M5CodeActionSideEffectClass::SingleFile,
        M5CodeActionBlockReason::NotBlocked,
    );
    input.detail_command_available = false;
    chip(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5DiagnosticChipConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    diagnostic_examples: Vec<M5ResolvedDiagnosticDecoration>,
    chip_examples: Vec<M5ResolvedCodeActionChip>,
) -> M5DiagnosticChipControlsRow {
    M5DiagnosticChipControlsRow {
        consumer_surface,
        qualification: M5EditorInlineQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EditorInlineDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EditorInlineRequiredLabel::Identity,
            M5EditorInlineRequiredLabel::State,
            M5EditorInlineRequiredLabel::KeyboardRoute,
            M5EditorInlineRequiredLabel::AnchorAndFreshness,
            M5EditorInlineRequiredLabel::ConfidenceAndSource,
        ],
        accessibility_routes: M5EditorInlineAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5DiagnosticChipAnatomyPart::ALL.to_vec(),
        export_fields: M5DiagnosticChipExportField::ALL.to_vec(),
        downgrade_triggers,
        diagnostic_examples,
        chip_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_REF,
            M5_DIAGNOSTIC_DECORATION_SCHEMA_REF,
            M5_CODE_ACTION_CHIP_SCHEMA_REF,
        ]),
        diagnostic_severity_or_source_encoded_by_color_alone: false,
        diagnostic_anchor_or_freshness_silently_drifts: false,
        inferred_or_blocked_fix_presented_as_exact_or_ready: false,
        code_action_bypasses_preview_or_apply_truth: false,
    }
}

fn controls_rows() -> Vec<M5DiagnosticChipControlsRow> {
    use M5EditorInlineConsumerSurface as C;
    use M5EditorInlineDowngradeTrigger as D;

    vec![
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor names problem severity and source on diagnostic decorations with no-color-only semantics and offers exact-versus-inferred code-action chips; both degrade honestly when severity is encoded by color alone or a fix posture is",
            "evidence:m5-diagnostic-chip-editor-ui:001",
            vec![
                D::TabMarkerDiagnosticColorOnly,
                D::InferredFixShownAsExact,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                dec_error_ls_clean(),
                dec_warning_linter_clean(),
                dec_severity_color_only(),
            ],
            vec![
                chip_exact_direct_clean(),
                chip_inferred_preview_clean(),
                chip_posture_color_only(),
            ],
        ),
        base_row(
            C::DiagnosticsUi,
            "Diagnostics surface owner",
            "The diagnostics surface correlates underlines, markers, and panel entries through one severity/source/freshness vocabulary and degrades honestly when severity, freshness, or a fix's apply scope cannot be resolved or a stale diagnostic is shown as current",
            "evidence:m5-diagnostic-chip-diagnostics-ui:001",
            vec![
                D::TabMarkerDiagnosticColorOnly,
                D::DiagnosticFreshnessUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                dec_info_compiler_clean(),
                dec_hint_testrunner_clean(),
                dec_stale_shown(),
                dec_severity_unresolved(),
                dec_freshness_unresolved(),
            ],
            vec![chip_heuristic_review_clean(), chip_scope_unresolved()],
        ),
        base_row(
            C::NotebookUi,
            "Notebook code-pane owner",
            "The notebook reuses the same diagnostic decoration and code-action chip grammar in code cells, discloses staleness rather than reading as current, and degrades honestly when an anchor silently drifts or a multi-file fix hides its side effect",
            "evidence:m5-diagnostic-chip-notebook-ui:001",
            vec![
                D::CommentAnchorDriftedSilently,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![dec_stale_disclosed_clean(), dec_anchor_drift_hidden()],
            vec![chip_exact_direct_clean(), chip_side_effect_hidden()],
        ),
        base_row(
            C::AiUi,
            "AI surface owner",
            "AI surfaces never imply native certainty for an imported diagnostic and never present an inferred fix as exact; blocked and external-state actions name their reason and side effect, degrading honestly when an imported diagnostic overstates certainty or an inferred fix reads as exact",
            "evidence:m5-diagnostic-chip-ai-ui:001",
            vec![
                D::InferredFixShownAsExact,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![dec_reanchored_ai_clean(), dec_imported_overstated()],
            vec![
                chip_blocked_clean(),
                chip_external_disclosed_clean(),
                chip_inferred_as_exact(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved diagnostic and fix truth, so a color-only severity, a broken Problems linkage, a hidden block reason, or a bypassed preview is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-diagnostic-chip-support-export:001",
            vec![
                D::GenericChromeWordingUsed,
                D::TabMarkerDiagnosticColorOnly,
                D::ProofStale,
            ],
            vec![
                dec_imported_distinguished_clean(),
                dec_identity_unstated(),
                dec_linkage_broken(),
            ],
            vec![
                chip_exact_direct_clean(),
                chip_blocked_reason_hidden(),
                chip_preview_bypassed(),
                chip_identity_unstated(),
                chip_posture_unresolved(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product editor owner",
            "In-product surfaces reuse the same diagnostic and fix grammar a user sees in the editor, always offering the command-backed detail/preview path and degrading honestly when the trace path is missing, the source or anchor is unresolved, or a fix's side-effect class is unresolved",
            "evidence:m5-diagnostic-chip-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::AnchorStateUnstated,
                D::ProofStale,
            ],
            vec![
                dec_error_ls_clean(),
                dec_detail_missing(),
                dec_anchor_unresolved(),
                dec_linkage_unresolved(),
                dec_source_unstated(),
            ],
            vec![
                chip_exact_direct_clean(),
                chip_detail_missing(),
                chip_side_effect_unresolved(),
            ],
        ),
    ]
}

fn governance_review() -> M5DiagnosticChipGovernanceReview {
    M5DiagnosticChipGovernanceReview {
        decoration_names_severity_source_and_freshness: true,
        decoration_severity_no_color_only: true,
        decoration_linkage_stable_to_problems_output_support: true,
        stale_diagnostics_never_shown_as_current: true,
        anchors_never_silently_drift: true,
        imported_diagnostics_never_overstate_certainty: true,
        chip_names_exact_versus_inferred_posture: true,
        inferred_fixes_never_presented_as_exact: true,
        chip_apply_scope_never_bypasses_preview: true,
        blocked_actions_always_carry_reason: true,
        multi_file_or_external_side_effects_disclosed: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5DiagnosticChipConsumerProjection {
    M5DiagnosticChipConsumerProjection {
        editor_surfaces_consume_diagnostic_and_chip_vocabulary: true,
        notebook_consumes_diagnostic_and_chip_vocabulary: true,
        ai_surfaces_consume_fix_posture_and_apply_scope_vocabulary: true,
        diagnostics_consume_severity_source_and_freshness_vocabulary: true,
        facts_trace_to_single_component_contract: true,
        support_export_reads_single_editor_source: true,
    }
}

fn proof_freshness() -> M5DiagnosticChipProofFreshness {
    M5DiagnosticChipProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DiagnosticChipReleasePosture {
    M5DiagnosticChipReleasePosture {
        proof_packet_ref: M5_DIAGNOSTIC_CHIP_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_DIAGNOSTIC_CHIP_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_REF,
        M5_DIAGNOSTIC_CHIP_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_DIAGNOSTIC_DECORATION_SCHEMA_REF,
        M5_CODE_ACTION_CHIP_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 diagnostic-decoration / code-action-chip controls packet.
pub fn seeded_m5_diagnostic_chip_controls() -> M5DiagnosticChipControlsPacket {
    M5DiagnosticChipControlsPacket::new(M5DiagnosticChipControlsPacketInput {
        packet_id: M5_DIAGNOSTIC_CHIP_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 diagnostic-decoration and code-action-chip controls with severity/source/freshness, exact-versus-inferred fix posture, preview-required apply scope, blocked-action reasons, and side-effect class aligned across editor, diagnostics, notebook, AI, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5DiagnosticChipVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the diagnostics-UI row is held at Beta pending severity/source/freshness parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_diagnostic_chip_controls_diagnostics_ui_beta_narrowed(
) -> M5DiagnosticChipControlsPacket {
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.packet_id =
        "m5-diagnostic-decoration-code-action-chip-controls:diagnostics-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::DiagnosticsUi)
        .expect("diagnostics-ui row present");
    row.qualification = M5EditorInlineQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-UI row is narrowed to Preview pending fix-posture / apply-scope parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_diagnostic_chip_controls_ai_ui_preview_narrowed() -> M5DiagnosticChipControlsPacket
{
    let mut packet = seeded_m5_diagnostic_chip_controls();
    packet.packet_id =
        "m5-diagnostic-decoration-code-action-chip-controls:ai-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::AiUi)
        .expect("ai-ui row present");
    row.qualification = M5EditorInlineQualificationClass::Preview;
    packet
}
