//! Deterministic claimed-stable matrix for safe-preview records.
//!
//! Every record here is a genuine projection of the live content-safety stack:
//! the suspicious-content findings are produced by
//! [`aureline_content_safety::detect_suspicious_content`] over fixed seed bytes,
//! the trust-class ladder and the representation actions are consumed from the
//! content-safety vocabulary, and the record is minted through the governed
//! [`SafePreviewRecord::build`] builder so a record can never drift from the
//! shared trust-class and representation vocabulary that ships.
//!
//! The matrix spans carriers (notification, activity center, browser handoff,
//! support export, screenshot/evidence) and trust-sensitive actions (install,
//! delete, open-external), and includes Stable rows plus four narrowed drills:
//!
//! - A **carrier-flatten drill** narrows because a support-export carrier
//!   flattens the warning into a generic preview.
//! - A **stricter-boundary drill** narrows because an install review does not
//!   show its stricter preview boundary before commit.
//! - A **reveal-missing drill** narrows because a rendered browser-handoff hides
//!   the raw-source reveal affordance, so rendered meaning cannot be told from
//!   source bytes.
//! - A **preview-surface posture** proves every pillar but binds a Help/About
//!   surface still in preview, so it narrows to Preview by its lowest binding
//!   surface marker instead of inheriting an adjacent green row.

use aureline_content_safety::{
    detect_suspicious_content, BodyPosture, DetectorOutcomeClass, RepresentationActionId,
    RepresentationClass, SuspiciousContentClass, TrustClass,
};

use crate::notification_attention_stable::model::{
    AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, RecoveryRouteRecord, StableClaimClass,
};

use super::model::{
    required_recovery_routes, CueCarrier, CueCarrierRow, PlatformConformanceRow,
    PlatformProfileClass, RepresentationChoiceRow, RepresentationCuesInput, SafePreviewA11yCues,
    SafePreviewClaimCeiling, SafePreviewInput, SafePreviewRecord, SafePreviewSurfaceProjectionInput,
    SafePreviewTruthSurface, SafePreviewUpstream, ShellAdjacentSurface, StricterBoundary,
    SuspiciousFindingRow, CONTENT_SAFETY_CONTRACT_REF,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/safe-preview";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/safe-preview";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-safe-preview";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-safe-preview";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-safe-preview";

const TRUST_CLASS_SCHEMA_VERSION: u32 = aureline_content_safety::records::TRUST_CLASS_SCHEMA_VERSION;
const REPRESENTATION_POLICY_SCHEMA_VERSION: u32 =
    aureline_content_safety::transfer::TEXT_REPRESENTATION_POLICY_SCHEMA_VERSION;

/// One scenario in the claimed-stable safe-preview matrix.
#[derive(Debug, Clone)]
pub struct SafePreviewScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Expected shell-adjacent surface family.
    pub expected_surface_class: ShellAdjacentSurface,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest binding surface).
    pub expected_surface_marker: LifecycleMarker,
    record: SafePreviewRecord,
}

impl SafePreviewScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> SafePreviewRecord {
        self.record.clone()
    }
}

/// The claimed-stable safe-preview matrix, in canonical order.
pub fn safe_preview_corpus() -> Vec<SafePreviewScenario> {
    vec![
        notification_bidi_warning(),
        activity_center_invisible_warning(),
        browser_handoff_open_external(),
        support_export_redacted(),
        screenshot_evidence_capture(),
        install_review_stricter_class(),
        delete_review_action(),
        support_export_flattened_preview_drill(),
        install_boundary_not_shown_drill(),
        rendered_reveal_missing_drill(),
        preview_help_surface_posture(),
    ]
}

// ---------------------------------------------------------------------------
// Projection helpers
// ---------------------------------------------------------------------------

/// Knobs that turn a Stable projection into an adversarial drill.
#[derive(Debug, Clone, Copy)]
struct ScenarioCfg {
    scenario_id: &'static str,
    fixture_stem: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    surface_class: ShellAdjacentSurface,
    renders_rich: bool,
    trust_class: TrustClass,
    detector_outcome: DetectorOutcomeClass,
    /// Seed bytes the detector runs over (None = clean, no findings).
    seed: Option<&'static str>,
    /// Whether the raw/reveal affordance is present (reveal-missing drill = false).
    raw_reveal_available: bool,
    /// A carrier that flattens to a generic preview (carrier-flatten drill).
    flatten_carrier: Option<CueCarrier>,
    /// Whether a trust-sensitive action shows the stricter boundary before commit.
    boundary_shows_before_commit: bool,
    /// Lifecycle marker for the Help/About binding surface (preview drill = Preview).
    help_about_marker: LifecycleMarker,
}

fn visibility_impact(class: SuspiciousContentClass) -> &'static str {
    match class {
        SuspiciousContentClass::BidiControl => "reorders_text",
        SuspiciousContentClass::InvisibleFormatting => "hides_codepoints",
        SuspiciousContentClass::MixedScriptConfusable
        | SuspiciousContentClass::WholeScriptConfusable => "looks_like_other_identifier",
        SuspiciousContentClass::RawRenderedDivergence => "rendered_differs_from_source",
    }
}

fn representation_label_for(class: SuspiciousContentClass) -> String {
    match class {
        SuspiciousContentClass::BidiControl => {
            "Bidi control — reorders display without changing source bytes".to_string()
        }
        SuspiciousContentClass::InvisibleFormatting => {
            "Invisible formatting — hides codepoints between glyphs".to_string()
        }
        SuspiciousContentClass::MixedScriptConfusable
        | SuspiciousContentClass::WholeScriptConfusable => {
            "Confusable identifier — looks like a different identifier".to_string()
        }
        SuspiciousContentClass::RawRenderedDivergence => {
            "Rendered view differs from source bytes".to_string()
        }
    }
}

/// Project the live detector output over seed bytes into surfaced finding rows.
fn findings_from_seed(seed: &str) -> Vec<SuspiciousFindingRow> {
    let detection = detect_suspicious_content(seed);
    detection
        .findings
        .iter()
        .map(|finding| SuspiciousFindingRow {
            finding_id: finding.finding_id.clone(),
            content_class: finding.class,
            visibility_impact: visibility_impact(finding.class).to_string(),
            representation_label: representation_label_for(finding.class),
            reveal_affordances: vec![
                "inline_marker".to_string(),
                "codepoint_inspector".to_string(),
                "raw_toggle".to_string(),
                "escaped_toggle".to_string(),
            ],
            raw_toggle_available: true,
            escaped_copy_available: true,
        })
        .collect()
}

fn choice(
    action_id: RepresentationActionId,
    representation_class: RepresentationClass,
    body_posture: BodyPosture,
    label: &str,
    raw_source_required: bool,
    active_content_removed: bool,
) -> RepresentationChoiceRow {
    RepresentationChoiceRow {
        action_id,
        representation_class,
        body_posture,
        label: label.to_string(),
        raw_source_required,
        active_content_removed,
    }
}

/// Representation choices in the content-safety vocabulary. Always offers a raw
/// path and a safe-inspection path; offers `Copy rendered` whenever a rendered
/// view exists.
fn representation_choices(renders_rich: bool) -> Vec<RepresentationChoiceRow> {
    let mut choices = vec![choice(
        RepresentationActionId::CopyRaw,
        RepresentationClass::Raw,
        BodyPosture::ExactSourceBytes,
        "Copy raw",
        true,
        false,
    )];
    if renders_rich {
        choices.push(choice(
            RepresentationActionId::CopyRendered,
            RepresentationClass::Rendered,
            BodyPosture::RenderedView,
            "Copy rendered",
            false,
            false,
        ));
    }
    choices.push(choice(
        RepresentationActionId::CopyEscaped,
        RepresentationClass::Escaped,
        BodyPosture::EscapedSourceText,
        "Copy escaped",
        true,
        false,
    ));
    choices.push(choice(
        RepresentationActionId::ExportSanitizedSnapshot,
        RepresentationClass::Sanitized,
        BodyPosture::SanitizedStaticSnapshot,
        "Export sanitized snapshot",
        false,
        true,
    ));
    if !renders_rich {
        choices.push(choice(
            RepresentationActionId::ExportMetadataOnly,
            RepresentationClass::BlockedMetadataOnly,
            BodyPosture::MetadataOnlyEnvelope,
            "Export metadata only",
            false,
            true,
        ));
    }
    choices
}

fn carrier_summary(carrier: CueCarrier, trust: TrustClass) -> String {
    format!(
        "{} keeps the {} trust-class badge, the representation label, and the suspicious-content warning.",
        carrier.label(),
        trust.as_str()
    )
}

fn cue_survival(trust: TrustClass, flatten: Option<CueCarrier>) -> Vec<CueCarrierRow> {
    CueCarrier::REQUIRED
        .iter()
        .map(|&carrier| {
            let flattened = flatten == Some(carrier);
            CueCarrierRow {
                carrier,
                preserves_trust_class: !flattened,
                preserves_representation_label: !flattened,
                preserves_suspicious_warning: !flattened,
                does_not_flatten_to_generic_preview: !flattened,
                carried_summary: if flattened {
                    format!(
                        "{} flattens the preview to a generic label and drops the warning.",
                        carrier.label()
                    )
                } else {
                    carrier_summary(carrier, trust)
                },
            }
        })
        .collect()
}

fn stricter_boundary(surface: ShellAdjacentSurface, shows_before_commit: bool) -> StricterBoundary {
    StricterBoundary {
        action: surface,
        enforces_stricter_preview_class: true,
        ordinary_browsing_class: TrustClass::SanitizedRich,
        enforced_preview_class: TrustClass::IsolatedRemoteActive,
        shows_boundary_before_commit: shows_before_commit,
        commit_blocked_until_acknowledged: shows_before_commit,
        boundary_disclosure: format!(
            "{} previews this content under the stricter isolated-remote class and blocks commit \
             until the suspicious-content warning is acknowledged.",
            surface.label()
        ),
    }
}

fn platform_conformance() -> Vec<PlatformConformanceRow> {
    vec![
        PlatformConformanceRow {
            profile: PlatformProfileClass::MacOs,
            profile_id: "macos_15_plus_universal".to_string(),
            covered: true,
            proof_ref: "aureline://fixture/ux-m4-safe-preview#macos".to_string(),
            named_behaviors: vec![
                "trust_class_badge_announced".to_string(),
                "raw_reveal_keyboard_reachable".to_string(),
                "warning_survives_notification_center".to_string(),
            ],
        },
        PlatformConformanceRow {
            profile: PlatformProfileClass::Windows,
            profile_id: "windows_11_x86_64".to_string(),
            covered: true,
            proof_ref: "aureline://fixture/ux-m4-safe-preview#windows".to_string(),
            named_behaviors: vec![
                "trust_class_badge_announced".to_string(),
                "raw_reveal_keyboard_reachable".to_string(),
                "warning_survives_action_center".to_string(),
            ],
        },
        PlatformConformanceRow {
            profile: PlatformProfileClass::Linux,
            profile_id: "linux_gnome_wayland_x86_64".to_string(),
            covered: true,
            proof_ref: "aureline://fixture/ux-m4-safe-preview#linux".to_string(),
            named_behaviors: vec![
                "trust_class_badge_announced".to_string(),
                "raw_reveal_keyboard_reachable".to_string(),
                "warning_survives_notification_daemon".to_string(),
            ],
        },
    ]
}

fn surface_projections(
    help_about_marker: LifecycleMarker,
) -> Vec<SafePreviewSurfaceProjectionInput> {
    SafePreviewTruthSurface::REQUIRED
        .iter()
        .map(|&surface| SafePreviewSurfaceProjectionInput {
            surface,
            surface_marker: if surface == SafePreviewTruthSurface::HelpAbout {
                help_about_marker
            } else {
                LifecycleMarker::Stable
            },
            reads_shared_record: true,
        })
        .collect()
}

fn accessibility(recovery_routes: &[RecoveryRouteRecord], trust: TrustClass) -> AccessibilityDisclosure {
    AccessibilityDisclosure {
        focus_order_index: 0,
        tab_stop_count: recovery_routes.len() as u32 + 1,
        row_narration: format!(
            "Safe preview, trust class {}; suspicious-content warning and representation label \
             announced; reveal raw source available.",
            trust.as_str()
        ),
        action_labels: recovery_routes
            .iter()
            .map(|route| route.action_label.clone())
            .collect(),
        layout_modes: LayoutMode::REQUIRED
            .iter()
            .map(|&mode| LayoutModeDisclosure {
                mode,
                row_narration_available: true,
                recovery_affordances_reachable: true,
            })
            .collect(),
    }
}

fn entry_routes(scenario_id: &str) -> Vec<EntryRouteRecord> {
    AttentionRouteSurface::REQUIRED
        .iter()
        .map(|&surface| EntryRouteRecord {
            surface,
            route_ref: format!("aureline://safe-preview/{scenario_id}#{}", surface.as_str()),
            keyboard_reachable: true,
            activates_same_item: true,
        })
        .collect()
}

fn a11y_cues(trust: TrustClass) -> SafePreviewA11yCues {
    SafePreviewA11yCues {
        warning_announced_not_color_only: true,
        representation_label_announced: true,
        trust_class_announced: true,
        reveal_affordance_keyboard_reachable: true,
        warning_summary_label: format!(
            "Suspicious content present; trust class {}; reveal raw source to inspect.",
            trust.as_str()
        ),
    }
}

fn upstream(case_refs: Vec<String>) -> SafePreviewUpstream {
    SafePreviewUpstream {
        content_safety_contract_ref: CONTENT_SAFETY_CONTRACT_REF.to_string(),
        trust_class_schema_version: TRUST_CLASS_SCHEMA_VERSION,
        representation_policy_schema_version: REPRESENTATION_POLICY_SCHEMA_VERSION,
        contributing_case_refs: case_refs,
    }
}

fn build_record(cfg: ScenarioCfg) -> SafePreviewRecord {
    let suspicious_findings = cfg.seed.map(findings_from_seed).unwrap_or_default();
    let has_finding = !suspicious_findings.is_empty();
    let recovery_routes = required_recovery_routes(has_finding);
    let case_refs: Vec<String> = suspicious_findings
        .iter()
        .map(|finding| format!("content-safety:case:{}", finding.finding_id))
        .collect();

    let stricter_boundary = if cfg.surface_class.is_trust_sensitive_action() {
        Some(stricter_boundary(cfg.surface_class, cfg.boundary_shows_before_commit))
    } else {
        None
    };

    let claim_ceiling = SafePreviewClaimCeiling {
        asserts_representation_cues_explicit: cfg.raw_reveal_available,
        asserts_suspicious_findings_surfaced: true,
        asserts_copy_export_labeled: true,
        asserts_cues_survive_all_carriers: cfg.flatten_carrier.is_none(),
        asserts_stricter_boundary_shown_before_commit: !cfg.surface_class.is_trust_sensitive_action()
            || cfg.boundary_shows_before_commit,
        asserts_accessibility_cues_complete: true,
        asserts_platform_conformance_complete: true,
    };

    let input = SafePreviewInput {
        record_id: cfg.scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: cfg.scenario_id.to_string(),
        posture_label: cfg.posture_label.to_string(),
        title: cfg.title.to_string(),
        summary: cfg.summary.to_string(),
        surface_class: cfg.surface_class,
        surface_id_ref: format!("safe-preview/{}", cfg.scenario_id),
        renders_rich_content: cfg.renders_rich,
        trust_class: cfg.trust_class,
        detector_outcome: cfg.detector_outcome,
        representation: RepresentationCuesInput {
            raw_reveal_available: cfg.raw_reveal_available,
            representation_label_present: true,
            representation_label: if cfg.renders_rich {
                "Source bytes (raw) — rendered view can differ; choose Copy raw or Copy rendered."
                    .to_string()
            } else {
                "Source bytes (raw) — escaped inspection available via Copy escaped.".to_string()
            },
        },
        suspicious_findings,
        representation_choices: representation_choices(cfg.renders_rich),
        cue_survival: cue_survival(cfg.trust_class, cfg.flatten_carrier),
        stricter_boundary,
        a11y_cues: a11y_cues(cfg.trust_class),
        platform_conformance: platform_conformance(),
        surface_projections: surface_projections(cfg.help_about_marker),
        claim_ceiling,
        recovery_routes: recovery_routes.clone(),
        routes: entry_routes(cfg.scenario_id),
        accessibility: accessibility(&recovery_routes, cfg.trust_class),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(case_refs),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![EVIDENCE_FIXTURE_REF.to_string(), EVIDENCE_ARTIFACT_REF.to_string()],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };

    SafePreviewRecord::build(input).unwrap_or_else(|err| {
        panic!("safe-preview scenario {} must build: {err}", cfg.scenario_id)
    })
}

fn scenario(
    cfg: ScenarioCfg,
    expected_claim_class: StableClaimClass,
    expected_qualifies_stable: bool,
    expected_surface_marker: LifecycleMarker,
) -> SafePreviewScenario {
    let record = build_record(cfg);
    SafePreviewScenario {
        scenario_id: cfg.scenario_id,
        fixture_filename: format!("{}.json", cfg.fixture_stem),
        expected_surface_class: cfg.surface_class,
        expected_claim_class,
        expected_qualifies_stable,
        expected_surface_marker,
        record,
    }
}

// ---------------------------------------------------------------------------
// Stable rows
// ---------------------------------------------------------------------------

fn notification_bidi_warning() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "notification_bidi_warning_stable",
            fixture_stem: "notification_bidi_warning_stable",
            posture_label: "Notification with a bidi-control warning",
            title: "A notification preview keeps the bidi-control warning and trust class",
            summary: "A build notification quoting a path with a bidi override keeps the RawText \
                      trust class, the suspicious-content warning, and the Copy raw / Copy escaped \
                      choices instead of flattening to a generic preview string.",
            surface_class: ShellAdjacentSurface::Notification,
            renders_rich: false,
            trust_class: TrustClass::RawText,
            detector_outcome: DetectorOutcomeClass::Sanitize,
            seed: Some("release\u{202E}gpj.exe"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn activity_center_invisible_warning() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "activity_center_invisible_warning_stable",
            fixture_stem: "activity_center_invisible_warning_stable",
            posture_label: "Activity-center row with an invisible-formatting warning",
            title: "An activity-center row keeps the invisible-formatting warning",
            summary: "A durable activity-center row quoting an identifier with a zero-width space \
                      keeps the RawText trust class, the warning, and the reveal affordances so the \
                      hidden codepoint never disappears into the row label.",
            surface_class: ShellAdjacentSurface::ActivityCenter,
            renders_rich: false,
            trust_class: TrustClass::RawText,
            detector_outcome: DetectorOutcomeClass::Sanitize,
            seed: Some("invoice\u{200B}_total"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn browser_handoff_open_external() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "browser_handoff_open_external_stable",
            fixture_stem: "browser_handoff_open_external_stable",
            posture_label: "Open-external handoff with a confusable URL",
            title: "An open-external handoff shows the stricter boundary before commit",
            summary: "Opening a confusable URL in the system browser enforces the stricter \
                      isolated-remote preview class, shows the boundary before commit, and keeps \
                      the mixed-script warning and representation choices explicit.",
            surface_class: ShellAdjacentSurface::OpenExternalHandoff,
            renders_rich: false,
            trust_class: TrustClass::IsolatedRemoteActive,
            detector_outcome: DetectorOutcomeClass::RouteToSystemBrowser,
            seed: Some("https://ex\u{0430}mple.com/login"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn support_export_redacted() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "support_export_redacted_stable",
            fixture_stem: "support_export_redacted_stable",
            posture_label: "Redacted support export with preserved warnings",
            title: "A redacted support export preserves the suspicious-content warning",
            summary: "A redacted support export of a path with a bidi override preserves the \
                      RawText trust class and the suspicious-content warning in its summary so the \
                      reviewer sees the same representation truth the shell showed.",
            surface_class: ShellAdjacentSurface::SupportExport,
            renders_rich: false,
            trust_class: TrustClass::RawText,
            detector_outcome: DetectorOutcomeClass::Sanitize,
            seed: Some("logs\u{202E}txt.path"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn screenshot_evidence_capture() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "screenshot_evidence_capture_stable",
            fixture_stem: "screenshot_evidence_capture_stable",
            posture_label: "Screenshot/evidence capture of a rendered preview",
            title: "A screenshot/evidence capture keeps Copy raw and Copy rendered explicit",
            summary: "A screenshot/evidence capture of a sanitized rich preview keeps the \
                      representation label and both Copy raw and Copy rendered choices explicit so \
                      the captured evidence cannot be mistaken for the source bytes.",
            surface_class: ShellAdjacentSurface::ScreenshotEvidence,
            renders_rich: true,
            trust_class: TrustClass::SanitizedRich,
            detector_outcome: DetectorOutcomeClass::Allow,
            seed: None,
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn install_review_stricter_class() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "install_review_stricter_class_stable",
            fixture_stem: "install_review_stricter_class_stable",
            posture_label: "Install review with a stricter preview class",
            title: "An install review shows the stricter preview class before commit",
            summary: "An extension install review of a package whose name hides a zero-width space \
                      enforces the stricter isolated-remote preview class, shows the boundary \
                      before the user installs, and keeps the warning and representation choices.",
            surface_class: ShellAdjacentSurface::InstallReview,
            renders_rich: false,
            trust_class: TrustClass::IsolatedRemoteActive,
            detector_outcome: DetectorOutcomeClass::Isolate,
            seed: Some("pkg\u{200B}name"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn delete_review_action() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "delete_review_action_stable",
            fixture_stem: "delete_review_action_stable",
            posture_label: "Delete review with a bidi-control warning",
            title: "A delete review shows the stricter boundary for confusable targets",
            summary: "A delete review of a filename with a bidi override enforces the stricter \
                      preview class, shows the boundary before commit, and keeps the warning so a \
                      user never deletes a target that is not what it appears to be.",
            surface_class: ShellAdjacentSurface::DeleteReview,
            renders_rich: false,
            trust_class: TrustClass::IsolatedRemoteActive,
            detector_outcome: DetectorOutcomeClass::Isolate,
            seed: Some("report\u{202E}cod.exe"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

// ---------------------------------------------------------------------------
// Narrowed drills
// ---------------------------------------------------------------------------

fn support_export_flattened_preview_drill() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "support_export_flattened_preview_drill",
            fixture_stem: "support_export_flattened_preview_drill",
            posture_label: "Support export that flattens the warning (narrowed)",
            title: "A support export that flattens the warning narrows below Stable",
            summary: "A support-export carrier that flattens the preview to a generic label drops \
                      the suspicious-content warning and the trust class, so the posture narrows \
                      below Stable with a named reason instead of inheriting a green row.",
            surface_class: ShellAdjacentSurface::SupportExport,
            renders_rich: false,
            trust_class: TrustClass::RawText,
            detector_outcome: DetectorOutcomeClass::Sanitize,
            seed: Some("notes\u{200B}export"),
            raw_reveal_available: true,
            flatten_carrier: Some(CueCarrier::SupportExport),
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn install_boundary_not_shown_drill() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "install_boundary_not_shown_drill",
            fixture_stem: "install_boundary_not_shown_drill",
            posture_label: "Install review hiding the stricter boundary (narrowed)",
            title: "An install review that hides the stricter boundary narrows below Stable",
            summary: "An install review that enforces a stricter preview class but does not show \
                      the boundary before commit lets the user act before seeing the stricter \
                      class, so the posture narrows below Stable with a named reason.",
            surface_class: ShellAdjacentSurface::InstallReview,
            renders_rich: false,
            trust_class: TrustClass::IsolatedRemoteActive,
            detector_outcome: DetectorOutcomeClass::Isolate,
            seed: Some("addon\u{200B}id"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: false,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn rendered_reveal_missing_drill() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "rendered_reveal_missing_drill",
            fixture_stem: "rendered_reveal_missing_drill",
            posture_label: "Rendered handoff hiding the raw reveal (narrowed)",
            title: "A rendered handoff that hides the raw reveal narrows below Stable",
            summary: "A browser handoff that renders rich content but hides the raw-source reveal \
                      affordance cannot show that rendered meaning differs from source bytes, so \
                      the posture narrows below Stable with a named reason.",
            surface_class: ShellAdjacentSurface::BrowserHandoff,
            renders_rich: true,
            trust_class: TrustClass::SanitizedRich,
            detector_outcome: DetectorOutcomeClass::Sanitize,
            seed: Some("preview\u{202E}cod"),
            raw_reveal_available: false,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Stable,
        },
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn preview_help_surface_posture() -> SafePreviewScenario {
    scenario(
        ScenarioCfg {
            scenario_id: "preview_help_surface_posture",
            fixture_stem: "preview_help_surface_posture",
            posture_label: "Notification binding a preview Help/About surface",
            title: "Every pillar holds but the Help/About surface is still in preview",
            summary: "A notification posture proves every safe-preview pillar but binds a \
                      Help/About surface still in preview, so it narrows to Preview by its lowest \
                      binding surface marker rather than inheriting an adjacent green row.",
            surface_class: ShellAdjacentSurface::Notification,
            renders_rich: false,
            trust_class: TrustClass::RawText,
            detector_outcome: DetectorOutcomeClass::Sanitize,
            seed: Some("alert\u{202E}txt"),
            raw_reveal_available: true,
            flatten_carrier: None,
            boundary_shows_before_commit: true,
            help_about_marker: LifecycleMarker::Preview,
        },
        StableClaimClass::Preview,
        false,
        LifecycleMarker::Preview,
    )
}
