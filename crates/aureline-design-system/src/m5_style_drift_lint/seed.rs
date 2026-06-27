//! Canonical seed builder for the M5 design-system style-drift-lint report.
//!
//! This builder is the single producer of the checked-in lint-report fixtures (the conformant
//! report plus the drift, waived, and expired-waiver drills), the lint-outcome proof, and the
//! release packet. The headless emitter and the inline tests both call it so the in-code report,
//! the schema fixtures, and the proof never drift. The conformant report consumes only governed
//! foundation tokens and binds every protected state with a label and a non-color cue, so the lint
//! pass it produces is green; the drills exercise the blocking gate and the time-bounded,
//! proof-tied waiver path.

use super::*;

/// Stable id of the canonical lint report.
pub const M5_STYLE_DRIFT_LINT_REPORT_ID: &str = "design-system:style-drift-lint:protected-surfaces";

/// Version of the canonical lint report.
pub const M5_STYLE_DRIFT_LINT_REPORT_VERSION: &str = "1.0.0";

/// Mint timestamp pinned by the seed builder.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

/// Timestamp the canonical report is evaluated as of (drives waiver expiry).
const EVALUATED_AT: &str = "2026-06-26T00:00:00Z";

const REPORT_OWNER_ROLE: &str = "Design system owner";
const SURFACE_OWNER_ROLE: &str = "Surface owner";
const REDACTION_CLASS: &str = "design_system_metadata_only";

/// A proof packet a waiver is tied to (lives under the design-system proof directory).
const WAIVER_PROOF_PACKET_REF: &str =
    "artifacts/release/m5-design-system-proof/style-drift-lint-outcome.json";

/// An active waiver expiry (after [`EVALUATED_AT`]).
const ACTIVE_EXPIRES_AT: &str = "2026-09-01T00:00:00Z";

/// An expired waiver expiry (before [`EVALUATED_AT`]).
const EXPIRED_EXPIRES_AT: &str = "2026-03-01T00:00:00Z";

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_STYLE_DRIFT_LINT_SCHEMA_REF.to_owned(),
        M5_STYLE_DRIFT_LINT_DOC_REF.to_owned(),
        M5_STYLE_DRIFT_LINT_PROOF_REF.to_owned(),
    ]
}

fn usage(
    usage_id: &str,
    role: &str,
    property: M5StylePropertyClass,
    token_ref: &str,
) -> M5TokenUsage {
    M5TokenUsage {
        usage_id: usage_id.to_owned(),
        role: role.to_owned(),
        property,
        token_ref: token_ref.to_owned(),
    }
}

/// Builds a conformant binding for a protected state: labeled, non-color, never spinner-only or
/// hover-only.
fn binding(
    surface: M5ProtectedSurfaceClass,
    state: CanonicalStateClass,
    screen_reader_label: &str,
    cues: &[NonColorCueClass],
) -> M5ProtectedStateBinding {
    M5ProtectedStateBinding {
        state_class: state,
        label_message_id: format!(
            "{}{}.state.{}",
            M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX,
            surface.as_str(),
            state.as_str()
        ),
        screen_reader_label: screen_reader_label.to_owned(),
        non_color_cues: cues.to_vec(),
        spinner_only: false,
        hover_only_critical_action: false,
        state_family_ref: format!(
            "design-system:foundation:component-state.{}",
            state.as_str()
        ),
    }
}

/// The four conformant protected-state bindings every surface declares.
fn conformant_bindings(surface: M5ProtectedSurfaceClass) -> Vec<M5ProtectedStateBinding> {
    use CanonicalStateClass as S;
    use NonColorCueClass as Cue;
    vec![
        binding(
            surface,
            S::Loading,
            "Loading",
            &[Cue::ProgressIndicator, Cue::LabelText],
        ),
        binding(surface, S::Pending, "Pending", &[Cue::LabelText, Cue::Icon]),
        binding(
            surface,
            S::Degraded,
            "Reduced capability",
            &[Cue::LabelText, Cue::Shape],
        ),
        binding(
            surface,
            S::Blocked,
            "Blocked",
            &[Cue::LabelText, Cue::LockOrShieldGlyph],
        ),
    ]
}

fn trust_prompt() -> M5ProtectedSurfaceLint {
    use M5StylePropertyClass as P;
    M5ProtectedSurfaceLint {
        surface_class: M5ProtectedSurfaceClass::TrustPrompt,
        surface_id: "design-system:protected-surface:trust_prompt".to_owned(),
        display_name: "Trust / permission prompt".to_owned(),
        owner_role: SURFACE_OWNER_ROLE.to_owned(),
        shell_surface_ref: "crates/aureline-shell".to_owned(),
        token_usages: vec![
            usage(
                "surface_bg",
                "Sheet surface",
                P::Background,
                "al.color.surface.raised",
            ),
            usage(
                "title_text",
                "Prompt title",
                P::Color,
                "al.color.text.primary",
            ),
            usage(
                "body_text",
                "Prompt body",
                P::Color,
                "al.color.text.secondary",
            ),
            usage("trust_glyph", "Lock glyph", P::Icon, "icon.metaphor.lock"),
            usage(
                "body_type",
                "Body typography",
                P::Typography,
                "typography.body",
            ),
            usage("sheet_pad", "Sheet padding", P::Spacing, "space.6"),
            usage(
                "enter_motion",
                "Sheet enter motion",
                P::Motion,
                "motion_standard",
            ),
        ],
        local_style_forks: Vec::new(),
        state_bindings: conformant_bindings(M5ProtectedSurfaceClass::TrustPrompt),
        waivers: Vec::new(),
    }
}

fn onboarding_flow() -> M5ProtectedSurfaceLint {
    use M5StylePropertyClass as P;
    M5ProtectedSurfaceLint {
        surface_class: M5ProtectedSurfaceClass::OnboardingFlow,
        surface_id: "design-system:protected-surface:onboarding_flow".to_owned(),
        display_name: "Onboarding flow".to_owned(),
        owner_role: SURFACE_OWNER_ROLE.to_owned(),
        shell_surface_ref: "crates/aureline-shell".to_owned(),
        token_usages: vec![
            usage(
                "surface_bg",
                "Step surface",
                P::Background,
                "al.color.surface.shell",
            ),
            usage(
                "heading_text",
                "Step heading",
                P::Color,
                "al.color.text.primary",
            ),
            usage("hint_text", "Step hint", P::Color, "al.color.text.muted"),
            usage("step_glyph", "Step glyph", P::Icon, "icon.metaphor.lock"),
            usage(
                "heading_type",
                "Heading typography",
                P::Typography,
                "typography.heading",
            ),
            usage("step_gap", "Step gap", P::Spacing, "space.4"),
            usage(
                "step_motion",
                "Step transition",
                P::Motion,
                "motion_standard",
            ),
        ],
        local_style_forks: Vec::new(),
        state_bindings: conformant_bindings(M5ProtectedSurfaceClass::OnboardingFlow),
        waivers: Vec::new(),
    }
}

fn notification_activity() -> M5ProtectedSurfaceLint {
    use M5StylePropertyClass as P;
    M5ProtectedSurfaceLint {
        surface_class: M5ProtectedSurfaceClass::NotificationActivity,
        surface_id: "design-system:protected-surface:notification_activity".to_owned(),
        display_name: "Notification / activity center".to_owned(),
        owner_role: SURFACE_OWNER_ROLE.to_owned(),
        shell_surface_ref: "crates/aureline-shell".to_owned(),
        token_usages: vec![
            usage(
                "row_bg",
                "Notification row surface",
                P::Background,
                "al.color.surface.raised",
            ),
            usage(
                "row_text",
                "Notification text",
                P::Color,
                "al.color.text.primary",
            ),
            usage(
                "meta_text",
                "Notification metadata",
                P::Color,
                "al.color.text.secondary",
            ),
            usage("row_glyph", "Status glyph", P::Icon, "icon.metaphor.lock"),
            usage(
                "row_type",
                "Row typography",
                P::Typography,
                "typography.body",
            ),
            usage("row_gap", "Row gap", P::Spacing, "space.2"),
            usage(
                "row_motion",
                "Row arrival motion",
                P::Motion,
                "motion_standard",
            ),
        ],
        local_style_forks: Vec::new(),
        state_bindings: conformant_bindings(M5ProtectedSurfaceClass::NotificationActivity),
        waivers: Vec::new(),
    }
}

fn embedded_boundary() -> M5ProtectedSurfaceLint {
    use M5StylePropertyClass as P;
    M5ProtectedSurfaceLint {
        surface_class: M5ProtectedSurfaceClass::EmbeddedBoundary,
        surface_id: "design-system:protected-surface:embedded_boundary".to_owned(),
        display_name: "Embedded-surface boundary".to_owned(),
        owner_role: SURFACE_OWNER_ROLE.to_owned(),
        shell_surface_ref: "crates/aureline-shell".to_owned(),
        token_usages: vec![
            usage(
                "bar_bg",
                "Boundary bar surface",
                P::Background,
                "al.color.surface.raised",
            ),
            usage(
                "bar_text",
                "Boundary label",
                P::Color,
                "al.color.text.primary",
            ),
            usage(
                "route_text",
                "Route / origin",
                P::Color,
                "al.color.text.secondary",
            ),
            usage("trust_glyph", "Trust glyph", P::Icon, "icon.metaphor.lock"),
            usage(
                "bar_type",
                "Bar typography",
                P::Typography,
                "typography.body",
            ),
            usage("bar_pad", "Bar padding", P::Spacing, "space.2"),
            usage(
                "bar_border",
                "Bar border",
                P::Border,
                "al.color.border.default",
            ),
        ],
        local_style_forks: Vec::new(),
        state_bindings: conformant_bindings(M5ProtectedSurfaceClass::EmbeddedBoundary),
        waivers: Vec::new(),
    }
}

/// Builds the canonical, conformant style-drift-lint report. Its lint pass is green
/// ([`GateStateClass::Pass`]).
pub fn seeded_m5_style_drift_lint_report() -> M5StyleDriftLintReport {
    M5StyleDriftLintReport {
        record_kind: M5_STYLE_DRIFT_LINT_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_STYLE_DRIFT_LINT_SCHEMA_VERSION,
        report_id: M5_STYLE_DRIFT_LINT_REPORT_ID.to_owned(),
        report_version: M5_STYLE_DRIFT_LINT_REPORT_VERSION.to_owned(),
        owner_role: REPORT_OWNER_ROLE.to_owned(),
        evaluated_at: EVALUATED_AT.to_owned(),
        surfaces: vec![
            trust_prompt(),
            onboarding_flow(),
            notification_activity(),
            embedded_boundary(),
        ],
        proof_lane_ref: M5_STYLE_DRIFT_LINT_PROOF_REF.to_owned(),
        release_packet_ref: M5_STYLE_DRIFT_LINT_RELEASE_PACKET_REF.to_owned(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        summary_message_id: format!(
            "{}{}.summary",
            M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX, M5_STYLE_DRIFT_LINT_REPORT_ID
        ),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Injects drift into the trust-prompt surface: two unmanaged token values, a forbidden local style
/// fork, a dropped `degraded` binding, and a `blocked` binding that is color-only, spinner-only, and
/// hover-only. The lint pass blocks ([`GateStateClass::Block`]).
pub fn seeded_m5_style_drift_lint_report_drift() -> M5StyleDriftLintReport {
    use M5StylePropertyClass as P;
    let mut report = seeded_m5_style_drift_lint_report();
    let trust = report
        .surfaces
        .iter_mut()
        .find(|s| s.surface_class == M5ProtectedSurfaceClass::TrustPrompt)
        .expect("trust surface present");

    // Unmanaged token values: a raw hex color and a raw dimension.
    trust.token_usages.push(usage(
        "accent_inline",
        "Inline accent color",
        P::Color,
        "#0A84FF",
    ));
    trust
        .token_usages
        .push(usage("pad_inline", "Inline padding", P::Spacing, "12px"));

    // A forbidden local style fork.
    trust.local_style_forks.push(M5LocalStyleFork {
        fork_id: "local_focus_ring".to_owned(),
        description: "Surface-local focus ring overriding the token".to_owned(),
        property: P::Border,
        replaces_token_ref: "al.color.border.focus".to_owned(),
    });

    // Drop the degraded binding so it is missing.
    trust
        .state_bindings
        .retain(|b| b.state_class != CanonicalStateClass::Degraded);

    // Regress the blocked binding to color-only, spinner-only, and hover-only.
    let blocked = trust
        .state_bindings
        .iter_mut()
        .find(|b| b.state_class == CanonicalStateClass::Blocked)
        .expect("blocked binding present");
    blocked.non_color_cues.clear();
    blocked.spinner_only = true;
    blocked.hover_only_critical_action = true;

    report
}

/// Builds a waiver targeting a check id on the trust-prompt surface.
fn trust_waiver(waiver_id: &str, check_id: &str, expires_at: &str) -> M5StyleDriftWaiver {
    M5StyleDriftWaiver {
        waiver_id: waiver_id.to_owned(),
        waived_check_id: check_id.to_owned(),
        waived_state_class: None,
        waived_subject_id: None,
        reason_message_id: format!(
            "{}trust_prompt.waiver.{}",
            M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX, waiver_id
        ),
        expires_at: expires_at.to_owned(),
        proof_packet_ref: WAIVER_PROOF_PACKET_REF.to_owned(),
    }
}

/// One waiver per drift check id introduced by [`seeded_m5_style_drift_lint_report_drift`].
fn drift_waivers(expires_at: &str) -> Vec<M5StyleDriftWaiver> {
    vec![
        trust_waiver("unmanaged_tokens", CHECK_UNMANAGED_TOKEN_VALUE, expires_at),
        trust_waiver("local_fork", CHECK_FORBIDDEN_LOCAL_STYLE_FORK, expires_at),
        trust_waiver(
            "missing_degraded",
            CHECK_MISSING_STATE_SEMANTIC_BINDING,
            expires_at,
        ),
        trust_waiver(
            "blocked_color_only",
            CHECK_COLOR_ONLY_STATE_MEANING,
            expires_at,
        ),
        trust_waiver("blocked_spinner_only", CHECK_SPINNER_ONLY_STATE, expires_at),
        trust_waiver(
            "blocked_hover_only",
            CHECK_HOVER_ONLY_CRITICAL_ACTION,
            expires_at,
        ),
    ]
}

/// The drift drill with active, proof-tied waivers covering every finding. The lint pass passes with
/// a disclosed gap ([`GateStateClass::PassWithDisclosedGap`]).
pub fn seeded_m5_style_drift_lint_report_waived() -> M5StyleDriftLintReport {
    let mut report = seeded_m5_style_drift_lint_report_drift();
    let trust = report
        .surfaces
        .iter_mut()
        .find(|s| s.surface_class == M5ProtectedSurfaceClass::TrustPrompt)
        .expect("trust surface present");
    trust.waivers = drift_waivers(ACTIVE_EXPIRES_AT);
    report
}

/// The drift drill with waivers that have already expired as of `evaluated_at`. The expired waivers
/// do not suppress their findings, so the lint pass still blocks ([`GateStateClass::Block`]).
pub fn seeded_m5_style_drift_lint_report_expired_waiver() -> M5StyleDriftLintReport {
    let mut report = seeded_m5_style_drift_lint_report_drift();
    let trust = report
        .surfaces
        .iter_mut()
        .find(|s| s.surface_class == M5ProtectedSurfaceClass::TrustPrompt)
        .expect("trust surface present");
    trust.waivers = drift_waivers(EXPIRED_EXPIRES_AT);
    report
}
