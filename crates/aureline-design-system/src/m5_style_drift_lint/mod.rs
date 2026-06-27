//! Token-consumption linting, forbidden-local-style-fork detection, and state-semantic audits for
//! the trust-bearing M5 shell surfaces.
//!
//! Where [`crate::m5_design_system_contract`] freezes the *governance matrix*,
//! [`crate::m5_foundation_package`] ships the *foundations* (tokens, density, motion, contrast, and
//! the controlled state vocabulary), [`crate::m5_component_manifest`] ships the durable *component
//! contracts*, and [`crate::m5_reference_layout`] ships the *reference layouts*, this module ships
//! the *conformance gate*: a checked-in [`M5StyleDriftLintReport`] that declares, per protected
//! surface, the tokens it consumes, the local style forks it carries (if any), and the
//! state-semantic bindings it renders, and a [`M5StyleDriftLintReport::lint`] pass that turns that
//! declaration into a blocking [`M5StyleDriftLintOutcome`].
//!
//! The lane covers the surfaces most exposed to surface-local styling and state drift — the trust
//! prompt, the onboarding flow, the notification / activity center, and the embedded-surface
//! boundary — and flags:
//!
//! - **unmanaged token values** — a token reference that is a raw literal (a hex color, a raw
//!   dimension, an `rgb(...)` / `hsl(...)` call) or that does not resolve into a governed foundation
//!   namespace, so a surface cannot bypass the canonical tokens with an inline value.
//! - **forbidden local style forks** — any declared local style override on a protected surface, so
//!   a trust-bearing flow cannot fork the design system in place without a reviewable failure.
//! - **missing state-semantic bindings** — a protected state (`loading`, `pending`, `degraded`,
//!   `blocked`) the surface does not bind to the controlled state family.
//!
//! and audits, for every protected state binding, that the state stays **labeled** (a visible label
//! and a screen-reader label), **non-color-only** (at least one [non-color cue](NonColorCueClass)),
//! **not spinner-only** (a `pending` / `degraded` / `blocked` state may not be carried by a spinner
//! alone), and **not hover-only** (no critical action or reason hidden behind hover).
//!
//! Findings can be suppressed only by an explicit [`M5StyleDriftWaiver`] that is **time-bounded**
//! (it carries an `expires_at` and stops suppressing once the report's `evaluated_at` reaches it)
//! and **tied to a design-system proof packet** (its `proof_packet_ref` must live under the
//! design-system proof directory). An expired or proof-less waiver does not suppress its finding, so
//! the surface still blocks. The [gate decision](M5StyleDriftLintOutcome::gate_decision) blocks
//! Stable promotion whenever a protected surface carries unwaived drift, which is the CI failure a
//! new local-style fork or unlabeled degraded state produces.
//!
//! The records are metadata-only truth packets: they carry semantic token *references* and message
//! *ids*, never raw color values, credential bodies, or provider payloads. The one place raw values
//! appear is inside a drift drill's [`M5TokenUsage::token_ref`], which the lint exists to reject.
//!
//! - Schema:
//!   [`schemas/design-system/m5-style-drift-lint.schema.json`](../../../../../schemas/design-system/m5-style-drift-lint.schema.json)
//! - Doc:
//!   [`docs/design-system/m5-style-drift-lint.md`](../../../../../docs/design-system/m5-style-drift-lint.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_style_drift_lint_report, seeded_m5_style_drift_lint_report_drift,
    seeded_m5_style_drift_lint_report_expired_waiver, seeded_m5_style_drift_lint_report_waived,
    M5_STYLE_DRIFT_LINT_REPORT_ID, M5_STYLE_DRIFT_LINT_REPORT_VERSION,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{CanonicalStateClass, FindingSeverity, GateStateClass, NonColorCueClass};

/// Record-kind tag carried by [`M5StyleDriftLintReport`].
pub const M5_STYLE_DRIFT_LINT_REPORT_RECORD_KIND: &str = "m5_design_system_style_drift_lint_report";

/// Record-kind tag carried by [`M5StyleDriftLintOutcome`].
pub const M5_STYLE_DRIFT_LINT_OUTCOME_RECORD_KIND: &str =
    "m5_design_system_style_drift_lint_outcome";

/// Record-kind tag carried by [`M5StyleDriftLintReleasePacket`].
pub const M5_STYLE_DRIFT_LINT_RELEASE_RECORD_KIND: &str =
    "m5_design_system_style_drift_lint_release";

/// Record-kind tag carried by [`M5StyleDriftFinding`].
pub const M5_STYLE_DRIFT_FINDING_RECORD_KIND: &str = "m5_design_system_style_drift_finding";

/// Schema version shared by the style-drift-lint records.
pub const M5_STYLE_DRIFT_LINT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the style-drift-lint boundary schema.
pub const M5_STYLE_DRIFT_LINT_SCHEMA_REF: &str =
    "schemas/design-system/m5-style-drift-lint.schema.json";

/// Repo-relative path of the style-drift-lint contract doc.
pub const M5_STYLE_DRIFT_LINT_DOC_REF: &str = "docs/design-system/m5-style-drift-lint.md";

/// Repo-relative path of the release-grade lint-outcome proof packet — the proof lane that blocks
/// drift for the lane.
pub const M5_STYLE_DRIFT_LINT_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/style-drift-lint-outcome.json";

/// Repo-relative path of the release-packet projection.
pub const M5_STYLE_DRIFT_LINT_RELEASE_REF: &str =
    "artifacts/release/m5-design-system-proof/style-drift-lint-release.json";

/// Release packet that keeps the lane current (shared with the contract matrix, foundation package,
/// component manifests, and reference layouts).
pub const M5_STYLE_DRIFT_LINT_RELEASE_PACKET_REF: &str = "evidence:m5-design-system-release-packet";

/// Repo-relative directory of the checked-in lint-report fixtures.
pub const M5_STYLE_DRIFT_LINT_DIR: &str = "fixtures/ui/m5-style-drift-lint/";

/// Repo-relative directory every waiver's proof packet must live under, so a waiver is tied to a
/// design-system proof packet rather than an arbitrary reference.
pub const M5_DESIGN_SYSTEM_PROOF_DIR: &str = "artifacts/release/m5-design-system-proof/";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX: &str = "design_system_style_drift.";

/// Stable check id for an unmanaged token value (a raw literal or an unmanaged namespace).
pub const CHECK_UNMANAGED_TOKEN_VALUE: &str = "style_drift.unmanaged_token_value";

/// Stable check id for a forbidden local style fork on a protected surface.
pub const CHECK_FORBIDDEN_LOCAL_STYLE_FORK: &str = "style_drift.forbidden_local_style_fork";

/// Stable check id for a missing state-semantic binding on a protected surface.
pub const CHECK_MISSING_STATE_SEMANTIC_BINDING: &str = "style_drift.missing_state_semantic_binding";

/// Stable check id for a protected state that is not labeled.
pub const CHECK_UNLABELED_STATE: &str = "state_semantic.unlabeled_state";

/// Stable check id for a protected state whose only cue is color.
pub const CHECK_COLOR_ONLY_STATE_MEANING: &str = "state_semantic.color_only_state_meaning";

/// Stable check id for a protected state carried by a spinner alone.
pub const CHECK_SPINNER_ONLY_STATE: &str = "state_semantic.spinner_only_state";

/// Stable check id for a protected state whose critical action or reason depends on hover only.
pub const CHECK_HOVER_ONLY_CRITICAL_ACTION: &str = "state_semantic.hover_only_critical_action";

/// Stable check id for a well-formed waiver that suppresses no finding (a stale waiver to prune).
pub const CHECK_WAIVER_UNUSED: &str = "waiver.unused";

/// The suppressible drift / state-semantic check ids, in stable order. A waiver may only target one
/// of these; [`CHECK_WAIVER_UNUSED`] is emitted by the lint and is not itself suppressible.
pub const SUPPRESSIBLE_CHECK_IDS: [&str; 7] = [
    CHECK_UNMANAGED_TOKEN_VALUE,
    CHECK_FORBIDDEN_LOCAL_STYLE_FORK,
    CHECK_MISSING_STATE_SEMANTIC_BINDING,
    CHECK_UNLABELED_STATE,
    CHECK_COLOR_ONLY_STATE_MEANING,
    CHECK_SPINNER_ONLY_STATE,
    CHECK_HOVER_ONLY_CRITICAL_ACTION,
];

/// Governed foundation token namespaces a managed token reference resolves into. A token whose
/// value does not begin with one of these is treated as unmanaged.
const MANAGED_TOKEN_PREFIXES: [&str; 18] = [
    "al.color.",
    "color.",
    "space.",
    "size.",
    "icon.",
    "typography.",
    "type.",
    "font.",
    "motion.",
    "motion_",
    "radius.",
    "elevation.",
    "shadow.",
    "z.",
    "border.",
    "density.",
    "contrast.",
    "state.",
];

/// The closed set of protected states every protected surface must bind and that the
/// state-semantic audit covers, in canonical order.
pub const PROTECTED_STATES: [CanonicalStateClass; 4] = [
    CanonicalStateClass::Loading,
    CanonicalStateClass::Pending,
    CanonicalStateClass::Degraded,
    CanonicalStateClass::Blocked,
];

/// True when `state` must carry an explicit affordance and so may not be represented by a spinner
/// alone. The `loading` state is the canonical spinner affordance and is exempt.
const fn spinner_only_forbidden(state: CanonicalStateClass) -> bool {
    matches!(
        state,
        CanonicalStateClass::Pending | CanonicalStateClass::Degraded | CanonicalStateClass::Blocked
    )
}

/// True when `state` is one of the [protected states](PROTECTED_STATES) the audit covers.
fn is_protected_state(state: CanonicalStateClass) -> bool {
    PROTECTED_STATES.contains(&state)
}

/// True when `value` resolves into a governed foundation token namespace.
pub fn token_value_is_managed(value: &str) -> bool {
    let value = value.trim();
    MANAGED_TOKEN_PREFIXES
        .iter()
        .any(|prefix| value.starts_with(prefix))
}

/// True when `value` looks like a raw, inline style literal (a hex color, a dimension, or a color
/// function) rather than a semantic token reference.
pub fn token_value_is_raw_literal(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() {
        return false;
    }
    if value.starts_with('#') || value.starts_with("0x") || value.starts_with("0X") {
        return true;
    }
    let lower = value.to_ascii_lowercase();
    for call in ["rgb(", "rgba(", "hsl(", "hsla(", "var(", "calc("] {
        if lower.starts_with(call) {
            return true;
        }
    }
    if value.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        for unit in ["px", "pt", "rem", "em", "vh", "vw", "%"] {
            if lower.ends_with(unit) {
                return true;
            }
        }
        if value.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return true;
        }
    }
    // A bare six-digit hex (e.g. "0A84FF") with no leading '#'.
    value.len() == 6 && value.chars().all(|c| c.is_ascii_hexdigit())
}

/// True when `value` is an unmanaged token value the lint must flag.
fn token_value_is_unmanaged(value: &str) -> bool {
    token_value_is_raw_literal(value) || !token_value_is_managed(value)
}

/// One protected, trust-bearing M5 shell surface the lint covers. The tokens map to the canonical
/// shell surfaces these flows render through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProtectedSurfaceClass {
    /// Trust / permission / capability prompt sheet.
    TrustPrompt,
    /// First-use and return-to-work onboarding flow.
    OnboardingFlow,
    /// Notification envelope and durable activity-center surface.
    NotificationActivity,
    /// Embedded-surface boundary indicator naming route, trust, and capability.
    EmbeddedBoundary,
}

impl M5ProtectedSurfaceClass {
    /// Every protected surface class, in declaration order. The report must cover one per class.
    pub const ALL: [Self; 4] = [
        Self::TrustPrompt,
        Self::OnboardingFlow,
        Self::NotificationActivity,
        Self::EmbeddedBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustPrompt => "trust_prompt",
            Self::OnboardingFlow => "onboarding_flow",
            Self::NotificationActivity => "notification_activity",
            Self::EmbeddedBoundary => "embedded_boundary",
        }
    }
}

/// The style property a token usage or local style fork governs. Informational classification that
/// keeps the lint output reviewable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StylePropertyClass {
    /// Foreground / text color.
    Color,
    /// Surface or fill background color.
    Background,
    /// Border color or treatment.
    Border,
    /// Spacing / sizing.
    Spacing,
    /// Typography (family, size, weight).
    Typography,
    /// Icon glyph or icon size.
    Icon,
    /// Motion duration / easing / posture.
    Motion,
    /// Elevation / shadow.
    Elevation,
    /// Corner radius.
    Radius,
}

impl M5StylePropertyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Color => "color",
            Self::Background => "background",
            Self::Border => "border",
            Self::Spacing => "spacing",
            Self::Typography => "typography",
            Self::Icon => "icon",
            Self::Motion => "motion",
            Self::Elevation => "elevation",
            Self::Radius => "radius",
        }
    }
}

/// One token a protected surface consumes for a style property.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TokenUsage {
    /// Stable usage id, unique within the surface.
    pub usage_id: String,
    /// What the token styles.
    pub role: String,
    /// The style property the token governs.
    pub property: M5StylePropertyClass,
    /// The token reference (or, in a drift drill, the raw literal the lint rejects).
    pub token_ref: String,
}

/// One declared local style override on a protected surface. Any entry is a forbidden fork unless
/// waived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalStyleFork {
    /// Stable fork id, unique within the surface.
    pub fork_id: String,
    /// What the override does.
    pub description: String,
    /// The style property the override forks.
    pub property: M5StylePropertyClass,
    /// The governed token the override forks away from.
    pub replaces_token_ref: String,
}

/// One protected state's semantic binding: how the surface renders the state so it stays labeled,
/// non-color-only, and explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProtectedStateBinding {
    /// The controlled state class.
    pub state_class: CanonicalStateClass,
    /// Governed visible-label message id; prefixed [`M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX`].
    pub label_message_id: String,
    /// Screen-reader label text for the state.
    pub screen_reader_label: String,
    /// Non-color cues that carry the state beyond hue.
    pub non_color_cues: Vec<NonColorCueClass>,
    /// True when the only affordance is a spinner.
    pub spinner_only: bool,
    /// True when a critical action or the state reason depends on hover only.
    pub hover_only_critical_action: bool,
    /// The foundation state family this binding resolves into.
    pub state_family_ref: String,
}

/// An explicit, time-bounded, proof-tied waiver that suppresses one drift / state-semantic finding
/// on its surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StyleDriftWaiver {
    /// Stable waiver id, unique within the surface.
    pub waiver_id: String,
    /// The suppressible check id this waiver targets (one of [`SUPPRESSIBLE_CHECK_IDS`]).
    pub waived_check_id: String,
    /// Optional state-class narrowing: when set, the waiver only suppresses findings for that state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waived_state_class: Option<CanonicalStateClass>,
    /// Optional subject narrowing: when set, the waiver only suppresses the finding for that subject
    /// id (a usage id, fork id, or `state:<state>` key).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waived_subject_id: Option<String>,
    /// Governed reason message id; prefixed [`M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// Expiry timestamp (ISO-8601 UTC). The waiver stops suppressing once `evaluated_at` reaches it.
    pub expires_at: String,
    /// Repo-relative design-system proof packet this waiver is tied to; must live under
    /// [`M5_DESIGN_SYSTEM_PROOF_DIR`].
    pub proof_packet_ref: String,
}

impl M5StyleDriftWaiver {
    /// True when the waiver carries every field correctly: a known check id, the governed reason
    /// prefix, an expiry, and a proof packet under the design-system proof directory. A
    /// malformed waiver can never suppress a finding.
    pub fn is_well_formed(&self) -> bool {
        !self.waiver_id.trim().is_empty()
            && SUPPRESSIBLE_CHECK_IDS.contains(&self.waived_check_id.as_str())
            && self
                .reason_message_id
                .starts_with(M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX)
            && !self.expires_at.trim().is_empty()
            && self
                .proof_packet_ref
                .starts_with(M5_DESIGN_SYSTEM_PROOF_DIR)
    }

    /// True when the waiver is well-formed and has not expired as of `evaluated_at`. ISO-8601 UTC
    /// timestamps order lexicographically, so a string comparison is correct here.
    pub fn is_active(&self, evaluated_at: &str) -> bool {
        self.is_well_formed() && evaluated_at < self.expires_at.as_str()
    }

    /// True when the waiver structurally targets `finding` (ignoring expiry), used to detect a
    /// stale waiver that suppresses nothing.
    fn matches_structurally(&self, finding: &M5StyleDriftFinding) -> bool {
        self.waived_check_id == finding.check_id
            && self
                .waived_state_class
                .map_or(true, |state| Some(state) == finding.state_class)
            && self
                .waived_subject_id
                .as_deref()
                .map_or(true, |subject| subject == finding.subject_id)
    }
}

/// One protected surface's declaration: the tokens it consumes, the local style forks it carries,
/// the state-semantic bindings it renders, and its waivers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProtectedSurfaceLint {
    /// The protected surface class.
    pub surface_class: M5ProtectedSurfaceClass,
    /// Stable surface id, unique within the report.
    pub surface_id: String,
    /// Human-readable surface name.
    pub display_name: String,
    /// Owner role accountable for the surface's conformance.
    pub owner_role: String,
    /// Repo-relative shell consumer reference this surface renders through.
    pub shell_surface_ref: String,
    /// The tokens the surface consumes.
    pub token_usages: Vec<M5TokenUsage>,
    /// The local style forks the surface declares (empty on a conformant surface).
    pub local_style_forks: Vec<M5LocalStyleFork>,
    /// The protected-state semantic bindings the surface renders.
    pub state_bindings: Vec<M5ProtectedStateBinding>,
    /// The explicit waivers in play for the surface.
    pub waivers: Vec<M5StyleDriftWaiver>,
}

impl M5ProtectedSurfaceLint {
    /// Finds the binding for a protected state.
    pub fn binding(&self, state: CanonicalStateClass) -> Option<&M5ProtectedStateBinding> {
        self.state_bindings.iter().find(|b| b.state_class == state)
    }
}

/// One drift / state-semantic finding emitted by the lint pass. A finding suppressed by an active
/// waiver carries the waiver id in [`waived_by`](Self::waived_by).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StyleDriftFinding {
    /// Record kind; must equal [`M5_STYLE_DRIFT_FINDING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable finding id.
    pub finding_id: String,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// The protected surface the finding belongs to.
    pub surface_class: M5ProtectedSurfaceClass,
    /// The state class the finding concerns, when state-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_class: Option<CanonicalStateClass>,
    /// The subject the finding concerns (a usage id, fork id, or `state:<state>` key).
    pub subject_id: String,
    /// Reviewer-facing note.
    pub note: String,
    /// The waiver id that suppressed the finding, when suppressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waived_by: Option<String>,
}

impl M5StyleDriftFinding {
    /// True when the finding is an error that is not suppressed by a waiver, so it blocks promotion.
    pub fn is_blocking(&self) -> bool {
        self.severity == FindingSeverity::Error && self.waived_by.is_none()
    }
}

/// A checked-in style-drift-lint report: the declaration the lint pass runs over.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StyleDriftLintReport {
    /// Record kind; must equal [`M5_STYLE_DRIFT_LINT_REPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STYLE_DRIFT_LINT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Report version (semver `MAJOR.MINOR.PATCH`).
    pub report_version: String,
    /// Owner role accountable for the report.
    pub owner_role: String,
    /// The timestamp the lint is evaluated as of; drives waiver expiry (ISO-8601 UTC).
    pub evaluated_at: String,
    /// The protected surfaces the report covers (one per [`M5ProtectedSurfaceClass`]).
    pub surfaces: Vec<M5ProtectedSurfaceLint>,
    /// Repo-relative proof lane that blocks drift.
    pub proof_lane_ref: String,
    /// Repo-relative release packet that keeps the report current.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable summary message id; prefixed [`M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5StyleDriftLintReport {
    /// Finds the lint declaration for a surface class.
    pub fn surface(&self, class: M5ProtectedSurfaceClass) -> Option<&M5ProtectedSurfaceLint> {
        self.surfaces.iter().find(|s| s.surface_class == class)
    }

    /// Total surfaces covered.
    pub fn total_surfaces(&self) -> usize {
        self.surfaces.len()
    }

    /// Validates the report's structural invariants, returning the violations (empty when valid).
    /// Structural validity is independent of whether the report carries drift: a drift drill is a
    /// valid report that the lint pass blocks.
    pub fn validate(&self) -> Vec<M5StyleDriftLintViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_STYLE_DRIFT_LINT_REPORT_RECORD_KIND {
            violations.push(M5StyleDriftLintViolation::WrongRecordKind);
        }
        if self.schema_version != M5_STYLE_DRIFT_LINT_SCHEMA_VERSION {
            violations.push(M5StyleDriftLintViolation::WrongSchemaVersion);
        }
        if self.report_id.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.proof_lane_ref.trim().is_empty()
            || self.release_packet_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5StyleDriftLintViolation::MissingIdentity);
        }
        if !is_semver(&self.report_version) {
            violations.push(M5StyleDriftLintViolation::BadReportVersion);
        }
        if !self
            .summary_message_id
            .starts_with(M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX)
        {
            violations.push(M5StyleDriftLintViolation::MessageIdPrefixMissing);
        }

        for required in [
            M5_STYLE_DRIFT_LINT_SCHEMA_REF,
            M5_STYLE_DRIFT_LINT_DOC_REF,
            M5_STYLE_DRIFT_LINT_PROOF_REF,
        ] {
            if !self.source_contract_refs.iter().any(|r| r == required) {
                violations.push(M5StyleDriftLintViolation::MissingSourceContracts);
                break;
            }
        }

        validate_surface_set(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 style drift lint report serializes"),
        ) {
            violations.push(M5StyleDriftLintViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// True when the report validates with no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON for the report.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 style drift lint report serializes")
    }

    /// Imports a report from JSON. The caller validates the returned report with [`Self::validate`].
    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    /// Runs the lint pass over the report, returning the blocking [`M5StyleDriftLintOutcome`].
    ///
    /// The pass emits one finding per unmanaged token value, forbidden local style fork, missing
    /// state binding, and state-semantic audit failure, then suppresses a finding when its surface
    /// carries an active (well-formed, unexpired) waiver targeting it. A well-formed waiver that
    /// suppresses nothing is reported as a non-blocking [`CHECK_WAIVER_UNUSED`] finding.
    pub fn lint(&self) -> M5StyleDriftLintOutcome {
        let mut findings: Vec<M5StyleDriftFinding> = Vec::new();
        let mut surface_gates: Vec<M5SurfaceGate> = Vec::new();

        for surface in &self.surfaces {
            let start = findings.len();
            lint_surface(surface, &mut findings);
            apply_waivers(surface, &self.evaluated_at, &mut findings[start..]);
            report_unused_waivers(surface, &mut findings);

            let scoped = &findings[start..];
            surface_gates.push(M5SurfaceGate {
                surface_class: surface.surface_class,
                surface_id: surface.surface_id.clone(),
                finding_count: scoped.len() as u32,
                blocking_finding_count: scoped.iter().filter(|f| f.is_blocking()).count() as u32,
                waived_finding_count: scoped.iter().filter(|f| f.waived_by.is_some()).count()
                    as u32,
                gate_decision: gate_for(scoped),
            });
        }

        let blocking_finding_count = findings.iter().filter(|f| f.is_blocking()).count() as u32;
        let waived_finding_count = findings.iter().filter(|f| f.waived_by.is_some()).count() as u32;
        let gate_decision = gate_for(&findings);

        M5StyleDriftLintOutcome {
            record_kind: M5_STYLE_DRIFT_LINT_OUTCOME_RECORD_KIND.to_owned(),
            schema_version: M5_STYLE_DRIFT_LINT_SCHEMA_VERSION,
            report_id: self.report_id.clone(),
            report_version: self.report_version.clone(),
            evaluated_at: self.evaluated_at.clone(),
            total_surfaces: self.total_surfaces() as u32,
            total_findings: findings.len() as u32,
            blocking_finding_count,
            waived_finding_count,
            gate_decision,
            surface_gates,
            findings,
            proof_lane_ref: self.proof_lane_ref.clone(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.outcome",
                M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX, self.report_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Projects the release packet: per-surface gate summaries and the overall gate decision, so a
    /// release record cites the lint result without carrying every finding.
    pub fn release_packet(&self) -> M5StyleDriftLintReleasePacket {
        let outcome = self.lint();
        M5StyleDriftLintReleasePacket {
            record_kind: M5_STYLE_DRIFT_LINT_RELEASE_RECORD_KIND.to_owned(),
            schema_version: M5_STYLE_DRIFT_LINT_SCHEMA_VERSION,
            report_id: self.report_id.clone(),
            report_version: self.report_version.clone(),
            evaluated_at: self.evaluated_at.clone(),
            total_surfaces: outcome.total_surfaces,
            total_findings: outcome.total_findings,
            blocking_finding_count: outcome.blocking_finding_count,
            waived_finding_count: outcome.waived_finding_count,
            gate_decision: outcome.gate_decision,
            surface_gates: outcome.surface_gates.clone(),
            proof_lane_ref: self.proof_lane_ref.clone(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.release",
                M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX, self.report_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }
}

/// Reads and validates the checked-in canonical lint-report fixture.
pub fn current_stable_m5_style_drift_lint_report(
) -> Result<M5StyleDriftLintReport, M5StyleDriftLintArtifactError> {
    let report: M5StyleDriftLintReport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-style-drift-lint/lint-report.json"
    )))
    .map_err(M5StyleDriftLintArtifactError::Parse)?;
    let violations = report.validate();
    if violations.is_empty() {
        Ok(report)
    } else {
        Err(M5StyleDriftLintArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading a checked-in lint-report export.
#[derive(Debug)]
pub enum M5StyleDriftLintArtifactError {
    /// The export failed to parse.
    Parse(serde_json::Error),
    /// The export failed validation.
    Validation(Vec<M5StyleDriftLintViolation>),
}

impl fmt::Display for M5StyleDriftLintArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(
                    formatter,
                    "m5 style drift lint report parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 style drift lint report failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5StyleDriftLintArtifactError {}

/// Validation failures emitted by [`M5StyleDriftLintReport::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5StyleDriftLintViolation {
    /// Report record kind is wrong.
    WrongRecordKind,
    /// Report schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The report version is not `MAJOR.MINOR.PATCH`.
    BadReportVersion,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A governed protected surface class has no covered surface.
    RequiredSurfaceClassMissing,
    /// Two surfaces share a class.
    DuplicateSurfaceClass,
    /// Two surfaces share a surface id.
    DuplicateSurfaceId,
    /// A surface is missing an identity field (id, name, owner, or shell ref).
    SurfaceIncomplete,
    /// A token usage has an empty field or a duplicate usage id.
    TokenUsageIncomplete,
    /// A local style fork has an empty field or a duplicate fork id.
    LocalStyleForkIncomplete,
    /// A state binding has an empty family ref, a duplicate state, or an unprefixed label id.
    StateBindingIncomplete,
    /// A waiver is malformed (bad id, unknown check, unprefixed reason, no expiry, or a proof packet
    /// outside the design-system proof directory) or duplicates a waiver id.
    WaiverMalformed,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5StyleDriftLintViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::BadReportVersion => "bad_report_version",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceClassMissing => "required_surface_class_missing",
            Self::DuplicateSurfaceClass => "duplicate_surface_class",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::SurfaceIncomplete => "surface_incomplete",
            Self::TokenUsageIncomplete => "token_usage_incomplete",
            Self::LocalStyleForkIncomplete => "local_style_fork_incomplete",
            Self::StateBindingIncomplete => "state_binding_incomplete",
            Self::WaiverMalformed => "waiver_malformed",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Outcome and release records.
// ---------------------------------------------------------------------------

/// The blocking outcome of a lint pass: every finding (suppressed or not), per-surface gates, and
/// the overall gate decision the release / public-truth automation reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StyleDriftLintOutcome {
    /// Record kind; must equal [`M5_STYLE_DRIFT_LINT_OUTCOME_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The report id this outcome projects.
    pub report_id: String,
    /// The report version.
    pub report_version: String,
    /// The timestamp the lint was evaluated as of.
    pub evaluated_at: String,
    /// Total surfaces covered.
    pub total_surfaces: u32,
    /// Total findings (suppressed and unsuppressed).
    pub total_findings: u32,
    /// Count of unwaived error findings (the blockers).
    pub blocking_finding_count: u32,
    /// Count of findings suppressed by an active waiver.
    pub waived_finding_count: u32,
    /// The overall gate decision.
    pub gate_decision: GateStateClass,
    /// Per-surface gate summaries, in report order.
    pub surface_gates: Vec<M5SurfaceGate>,
    /// Every finding, in surface then emission order.
    pub findings: Vec<M5StyleDriftFinding>,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5StyleDriftLintOutcome {
    /// True when the outcome blocks Stable promotion (an unwaived protected-surface drift).
    pub fn blocks_stable_promotion(&self) -> bool {
        self.gate_decision == GateStateClass::Block
    }

    /// The surface ids whose gate blocks promotion.
    pub fn blocked_surface_ids(&self) -> Vec<&str> {
        self.surface_gates
            .iter()
            .filter(|g| g.gate_decision == GateStateClass::Block)
            .map(|g| g.surface_id.as_str())
            .collect()
    }

    /// Deterministic export-safe JSON for the outcome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 style drift lint outcome serializes")
    }
}

/// One surface's gate summary inside an outcome or release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceGate {
    /// The protected surface class.
    pub surface_class: M5ProtectedSurfaceClass,
    /// The surface id.
    pub surface_id: String,
    /// Total findings for the surface.
    pub finding_count: u32,
    /// Count of unwaived error findings for the surface.
    pub blocking_finding_count: u32,
    /// Count of findings suppressed by an active waiver for the surface.
    pub waived_finding_count: u32,
    /// The surface's gate decision.
    pub gate_decision: GateStateClass,
}

/// Release-packet projection of a lint report: per-surface gates and the overall decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StyleDriftLintReleasePacket {
    /// Record kind; must equal [`M5_STYLE_DRIFT_LINT_RELEASE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The report id this release record projects.
    pub report_id: String,
    /// The report version.
    pub report_version: String,
    /// The timestamp the lint was evaluated as of.
    pub evaluated_at: String,
    /// Total surfaces covered.
    pub total_surfaces: u32,
    /// Total findings (suppressed and unsuppressed).
    pub total_findings: u32,
    /// Count of unwaived error findings.
    pub blocking_finding_count: u32,
    /// Count of findings suppressed by an active waiver.
    pub waived_finding_count: u32,
    /// The overall gate decision.
    pub gate_decision: GateStateClass,
    /// Per-surface gate summaries, in report order.
    pub surface_gates: Vec<M5SurfaceGate>,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5StyleDriftLintReleasePacket {
    /// Deterministic export-safe JSON for the release packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 style drift lint release packet serializes")
    }
}

// ---------------------------------------------------------------------------
// Lint pass.
// ---------------------------------------------------------------------------

fn finding(
    severity: FindingSeverity,
    check_id: &str,
    surface: M5ProtectedSurfaceClass,
    state_class: Option<CanonicalStateClass>,
    subject_id: impl Into<String>,
    note: impl Into<String>,
) -> M5StyleDriftFinding {
    let subject_id = subject_id.into();
    let state_token = state_class.map(|s| s.as_str()).unwrap_or("none");
    M5StyleDriftFinding {
        record_kind: M5_STYLE_DRIFT_FINDING_RECORD_KIND.to_owned(),
        schema_version: M5_STYLE_DRIFT_LINT_SCHEMA_VERSION,
        finding_id: format!(
            "{}:{}:{}:{}",
            surface.as_str(),
            check_id,
            state_token,
            subject_id
        ),
        severity,
        check_id: check_id.to_owned(),
        surface_class: surface,
        state_class,
        subject_id,
        note: note.into(),
        waived_by: None,
    }
}

/// Emits the drift and state-semantic findings for one surface (no waivers applied yet).
fn lint_surface(surface: &M5ProtectedSurfaceLint, findings: &mut Vec<M5StyleDriftFinding>) {
    let class = surface.surface_class;

    for usage in &surface.token_usages {
        if token_value_is_unmanaged(&usage.token_ref) {
            findings.push(finding(
                FindingSeverity::Error,
                CHECK_UNMANAGED_TOKEN_VALUE,
                class,
                None,
                &usage.usage_id,
                format!(
                    "token usage `{}` resolves to unmanaged value `{}`; use a governed foundation token",
                    usage.usage_id, usage.token_ref
                ),
            ));
        }
    }

    for fork in &surface.local_style_forks {
        findings.push(finding(
            FindingSeverity::Error,
            CHECK_FORBIDDEN_LOCAL_STYLE_FORK,
            class,
            None,
            &fork.fork_id,
            format!(
                "local style fork `{}` forks `{}`; protected surfaces may not fork the design system",
                fork.fork_id, fork.replaces_token_ref
            ),
        ));
    }

    for state in PROTECTED_STATES {
        if surface.binding(state).is_none() {
            findings.push(finding(
                FindingSeverity::Error,
                CHECK_MISSING_STATE_SEMANTIC_BINDING,
                class,
                Some(state),
                format!("state:{}", state.as_str()),
                format!(
                    "protected state `{}` is not bound to the controlled state family",
                    state.as_str()
                ),
            ));
        }
    }

    for binding in &surface.state_bindings {
        let state = binding.state_class;
        if !is_protected_state(state) {
            continue;
        }
        let subject = format!("state:{}", state.as_str());
        if binding.label_message_id.trim().is_empty()
            || binding.screen_reader_label.trim().is_empty()
        {
            findings.push(finding(
                FindingSeverity::Error,
                CHECK_UNLABELED_STATE,
                class,
                Some(state),
                &subject,
                format!(
                    "protected state `{}` is not labeled (a visible label and screen-reader label are required)",
                    state.as_str()
                ),
            ));
        }
        if binding.non_color_cues.is_empty() {
            findings.push(finding(
                FindingSeverity::Error,
                CHECK_COLOR_ONLY_STATE_MEANING,
                class,
                Some(state),
                &subject,
                format!(
                    "protected state `{}` carries meaning by color only; a non-color cue is required",
                    state.as_str()
                ),
            ));
        }
        if binding.spinner_only && spinner_only_forbidden(state) {
            findings.push(finding(
                FindingSeverity::Error,
                CHECK_SPINNER_ONLY_STATE,
                class,
                Some(state),
                &subject,
                format!(
                    "protected state `{}` is represented by a spinner alone",
                    state.as_str()
                ),
            ));
        }
        if binding.hover_only_critical_action {
            findings.push(finding(
                FindingSeverity::Error,
                CHECK_HOVER_ONLY_CRITICAL_ACTION,
                class,
                Some(state),
                &subject,
                format!(
                    "protected state `{}` hides a critical action or reason behind hover only",
                    state.as_str()
                ),
            ));
        }
    }
}

/// Suppresses each finding in `scoped` for which the surface carries an active waiver, recording the
/// waiver id on the finding.
fn apply_waivers(
    surface: &M5ProtectedSurfaceLint,
    evaluated_at: &str,
    scoped: &mut [M5StyleDriftFinding],
) {
    for finding in scoped.iter_mut() {
        if let Some(waiver) = surface
            .waivers
            .iter()
            .find(|w| w.is_active(evaluated_at) && w.matches_structurally(finding))
        {
            finding.waived_by = Some(waiver.waiver_id.clone());
        }
    }
}

/// Emits a non-blocking [`CHECK_WAIVER_UNUSED`] finding for every well-formed waiver that matches no
/// finding on the surface, so reviewers prune stale waivers. A malformed waiver is caught by
/// validation, not here.
fn report_unused_waivers(
    surface: &M5ProtectedSurfaceLint,
    findings: &mut Vec<M5StyleDriftFinding>,
) {
    let surface_findings: Vec<M5StyleDriftFinding> = findings
        .iter()
        .filter(|f| f.surface_class == surface.surface_class && f.check_id != CHECK_WAIVER_UNUSED)
        .cloned()
        .collect();
    for waiver in &surface.waivers {
        if !waiver.is_well_formed() {
            continue;
        }
        let matches_any = surface_findings
            .iter()
            .any(|f| waiver.matches_structurally(f));
        if !matches_any {
            findings.push(finding(
                FindingSeverity::Warning,
                CHECK_WAIVER_UNUSED,
                surface.surface_class,
                waiver.waived_state_class,
                &waiver.waiver_id,
                format!(
                    "waiver `{}` for `{}` suppresses no finding; prune it",
                    waiver.waiver_id, waiver.waived_check_id
                ),
            ));
        }
    }
}

/// Resolves the gate decision for a finding slice: [`GateStateClass::Block`] on any unwaived error,
/// [`GateStateClass::PassWithDisclosedGap`] when only waived errors remain,
/// [`GateStateClass::Warn`] for warnings only, else [`GateStateClass::Pass`].
fn gate_for(findings: &[M5StyleDriftFinding]) -> GateStateClass {
    let unwaived_errors = findings.iter().any(M5StyleDriftFinding::is_blocking);
    let waived = findings.iter().any(|f| f.waived_by.is_some());
    let warnings = findings
        .iter()
        .any(|f| f.severity == FindingSeverity::Warning);
    if unwaived_errors {
        GateStateClass::Block
    } else if waived {
        GateStateClass::PassWithDisclosedGap
    } else if warnings {
        GateStateClass::Warn
    } else {
        GateStateClass::Pass
    }
}

// ---------------------------------------------------------------------------
// Validation helpers.
// ---------------------------------------------------------------------------

fn validate_surface_set(
    report: &M5StyleDriftLintReport,
    violations: &mut Vec<M5StyleDriftLintViolation>,
) {
    let present: BTreeSet<M5ProtectedSurfaceClass> =
        report.surfaces.iter().map(|s| s.surface_class).collect();
    for required in M5ProtectedSurfaceClass::ALL {
        if !present.contains(&required) {
            violations.push(M5StyleDriftLintViolation::RequiredSurfaceClassMissing);
            break;
        }
    }
    if present.len() != report.surfaces.len() {
        violations.push(M5StyleDriftLintViolation::DuplicateSurfaceClass);
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &report.surfaces {
        if !seen_ids.insert(surface.surface_id.as_str()) {
            violations.push(M5StyleDriftLintViolation::DuplicateSurfaceId);
        }
        validate_surface(surface, violations);
    }
}

fn validate_surface(
    surface: &M5ProtectedSurfaceLint,
    violations: &mut Vec<M5StyleDriftLintViolation>,
) {
    if surface.surface_id.trim().is_empty()
        || surface.display_name.trim().is_empty()
        || surface.owner_role.trim().is_empty()
        || surface.shell_surface_ref.trim().is_empty()
    {
        violations.push(M5StyleDriftLintViolation::SurfaceIncomplete);
    }

    let mut usage_ids: BTreeSet<&str> = BTreeSet::new();
    let mut usage_bad = false;
    for usage in &surface.token_usages {
        if !usage_ids.insert(usage.usage_id.as_str()) {
            usage_bad = true;
        }
        if usage.usage_id.trim().is_empty()
            || usage.role.trim().is_empty()
            || usage.token_ref.trim().is_empty()
        {
            usage_bad = true;
        }
    }
    if usage_bad {
        violations.push(M5StyleDriftLintViolation::TokenUsageIncomplete);
    }

    let mut fork_ids: BTreeSet<&str> = BTreeSet::new();
    let mut fork_bad = false;
    for fork in &surface.local_style_forks {
        if !fork_ids.insert(fork.fork_id.as_str()) {
            fork_bad = true;
        }
        if fork.fork_id.trim().is_empty()
            || fork.description.trim().is_empty()
            || fork.replaces_token_ref.trim().is_empty()
        {
            fork_bad = true;
        }
    }
    if fork_bad {
        violations.push(M5StyleDriftLintViolation::LocalStyleForkIncomplete);
    }

    let mut states: BTreeSet<CanonicalStateClass> = BTreeSet::new();
    let mut binding_bad = false;
    for binding in &surface.state_bindings {
        if !states.insert(binding.state_class) {
            binding_bad = true;
        }
        if binding.state_family_ref.trim().is_empty()
            || (!binding.label_message_id.trim().is_empty()
                && !binding
                    .label_message_id
                    .starts_with(M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX))
        {
            binding_bad = true;
        }
    }
    if binding_bad {
        violations.push(M5StyleDriftLintViolation::StateBindingIncomplete);
    }

    let mut waiver_ids: BTreeSet<&str> = BTreeSet::new();
    let mut waiver_bad = false;
    for waiver in &surface.waivers {
        if !waiver_ids.insert(waiver.waiver_id.as_str()) {
            waiver_bad = true;
        }
        if !waiver.is_well_formed() {
            waiver_bad = true;
        }
    }
    if waiver_bad {
        violations.push(M5StyleDriftLintViolation::WaiverMalformed);
    }
}

/// True when `value` is a `MAJOR.MINOR.PATCH` numeric semver.
fn is_semver(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material (credential bodies,
/// raw provider payloads). These records are metadata-only by construction; this is a
/// defense-in-depth scan over the serialized export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 6] = [
        "api_key",
        "authorization",
        "password",
        "secret",
        "access_token",
        "raw_payload",
    ];
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if FORBIDDEN_KEYS.contains(&key.to_lowercase().as_str()) {
                    return true;
                }
                if json_contains_forbidden_boundary_material(child) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
