//! Beta debugger and execution-context support matrix per launch-language
//! wedge.
//!
//! The beta program claims two launch-language wedges: `python` and
//! `typescript_javascript`. This module declares the typed support matrix
//! every migration, partner, and release packet consumes when answering
//! "which run / test / debug / execution-context capabilities does Aureline
//! support on this wedge today, and how does support narrow when the runtime
//! cannot honour the claim?" The canonical matrix is exposed by
//! [`SupportMatrixBetaManifest::canonical`] so the markdown matrix, the
//! reviewer doc, the shell panel, and the integration-test fixtures all read
//! the same closed vocabulary instead of forking a parallel dialect.
//!
//! Adding a wedge, lane, support class, or downgrade rule is a vocabulary
//! change that MUST update the canonical manifest, the schema, the reviewer
//! doc, the markdown matrix, and the checked-in fixtures together.
//!
//! The machine-readable boundary lives at
//! [`/schemas/runtime/support_matrix_beta.schema.json`](../../../../schemas/runtime/support_matrix_beta.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/runtime/m3/language_runtime_support_beta.md`](../../../../docs/runtime/m3/language_runtime_support_beta.md).
//! The publishable matrix lives at
//! [`/artifacts/compat/m3/debug_execution_matrix.md`](../../../../artifacts/compat/m3/debug_execution_matrix.md).

use serde::{Deserialize, Serialize};

/// Schema version of the beta debugger and execution-context support matrix.
pub const SUPPORT_MATRIX_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one matrix wedge row.
pub const SUPPORT_MATRIX_BETA_WEDGE_ROW_RECORD_KIND: &str =
    "support_matrix_beta_wedge_row_record";

/// Stable record-kind tag for the canonical manifest.
pub const SUPPORT_MATRIX_BETA_MANIFEST_RECORD_KIND: &str =
    "support_matrix_beta_manifest_record";

/// Stable record-kind tag for one partner-input fixture record.
pub const SUPPORT_MATRIX_BETA_WEDGE_INPUT_RECORD_KIND: &str =
    "support_matrix_beta_wedge_input_record";

/// Stable record-kind tag for the support-export bundle.
pub const SUPPORT_MATRIX_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "support_matrix_beta_support_export_record";

/// Closed vocabulary of launch-language wedges the beta program claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportMatrixWedgeId {
    /// Python service or data-app wedge.
    Python,
    /// TypeScript / JavaScript web-app or service wedge.
    TypescriptJavascript,
}

impl SupportMatrixWedgeId {
    /// All claimed wedges in canonical iteration order.
    pub const ALL: [Self; 2] = [Self::Python, Self::TypescriptJavascript];

    /// Stable token recorded in records, schemas, and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypescriptJavascript => "typescript_javascript",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Python => "Python service / data app",
            Self::TypescriptJavascript => "TypeScript / JavaScript web app or service",
        }
    }
}

/// Closed support-class vocabulary used by every column of the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportMatrixClass {
    /// Capability is exercised by the claimed beta surfaces, fixtures, and
    /// integration tests; protected dispatch is allowed when context truth
    /// agrees with the stored binding.
    Supported,
    /// Capability is wired and inspectable but not yet exercised by the
    /// claimed beta workflows; rows render but protected dispatch requires
    /// review.
    Preview,
    /// Capability is narrowly available (inspection only, imported metadata
    /// only) and cannot dispatch mutating or privileged work.
    Limited,
    /// Capability is explicitly out of beta scope; protected dispatch fails
    /// closed and the row never advertises auto-dispatch.
    Unsupported,
}

impl SupportMatrixClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Preview => "preview",
            Self::Limited => "limited",
            Self::Unsupported => "unsupported",
        }
    }

    /// Reviewer-facing label used by the docs and the shell panel.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supported => "Supported",
            Self::Preview => "Preview",
            Self::Limited => "Limited",
            Self::Unsupported => "Unsupported",
        }
    }

    /// True when the class permits protected dispatch on a fresh, in-sync
    /// resolved context. `Preview` and `Limited` rows MUST NOT auto-dispatch
    /// protected work without explicit review.
    pub const fn allows_protected_dispatch(self) -> bool {
        matches!(self, Self::Supported)
    }
}

/// Closed vocabulary of execution-context beta lanes. Mirrors the lane manifest
/// on [`crate::execution_context::beta::ExecutionContextBetaLane`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportMatrixContextLane {
    /// Local-host lane (terminal, task, test, debug, ai_tool_call).
    LocalHost,
    /// Remote-attach lane (SSH, notebook-kernel-remote).
    RemoteAttach,
    /// Container / devcontainer lane.
    Container,
    /// Request-workspace lane (managed workspace, prebuild runtime, AI
    /// sandbox).
    RequestWorkspace,
}

impl SupportMatrixContextLane {
    /// All execution-context lanes in canonical iteration order.
    pub const ALL: [Self; 4] = [
        Self::LocalHost,
        Self::RemoteAttach,
        Self::Container,
        Self::RequestWorkspace,
    ];

    /// Stable token recorded in records, schemas, and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::RemoteAttach => "remote_attach",
            Self::Container => "container",
            Self::RequestWorkspace => "request_workspace",
        }
    }
}

/// Closed vocabulary of downgrade rules every wedge row carries. Rows MUST
/// quote the exact tokens from this set; free-form downgrade prose is
/// forbidden so support evidence cannot disagree with the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportMatrixDowngradeRule {
    /// Adapter accepted the launch but dropped a requested capability; the
    /// session continues with the dropped capability recorded on the
    /// snapshot.
    NarrowLaunchOnAdapterCapabilityDrop,
    /// Adapter accepted attach but dropped a requested capability; the
    /// session narrows to inspect-only until the capability returns.
    NarrowAttachToInspectOnlyOnCapabilityDrop,
    /// Test attempt's framework is outside the claimed coverage manifest;
    /// rows render as unclaimed instead of dispatching.
    BlockOnUnclaimedTestFramework,
    /// Target class falls outside the claimed lane vocabulary; protected
    /// dispatch fails closed instead of inferring a fallback lane.
    BlockOnUnclaimedTargetClass,
    /// Stored launch / debug ticket disagrees with the freshly resolved
    /// execution context; dispatch MUST be re-authorised.
    BlockProtectedDispatchOnTicketDrift,
    /// Environment capsule advanced past the stored hash, or drift state
    /// regressed; dispatch is blocked until the capsule is reconciled.
    BlockProtectedDispatchOnCapsuleDrift,
    /// Trust regressed from `trusted` to `restricted` or `pending`; protected
    /// dispatch is blocked rather than narrowed.
    BlockProtectedDispatchOnTrustStateRegression,
    /// Policy epoch on the stored ticket is older than the freshly resolved
    /// epoch; dispatch is blocked until the policy is reviewed.
    BlockProtectedDispatchOnPolicyEpochRegression,
    /// Target became unreachable; the matrix row narrows to evidence-only
    /// and refuses protected dispatch.
    BlockOnTargetUnreachable,
    /// Adapter initialisation timed out or the negotiation refused a
    /// required capability; the session terminates with a typed reason
    /// instead of partially launching.
    BlockOnAdapterNegotiationRefused,
}

impl SupportMatrixDowngradeRule {
    /// All canonical downgrade rules in iteration order.
    pub const ALL: [Self; 10] = [
        Self::NarrowLaunchOnAdapterCapabilityDrop,
        Self::NarrowAttachToInspectOnlyOnCapabilityDrop,
        Self::BlockOnUnclaimedTestFramework,
        Self::BlockOnUnclaimedTargetClass,
        Self::BlockProtectedDispatchOnTicketDrift,
        Self::BlockProtectedDispatchOnCapsuleDrift,
        Self::BlockProtectedDispatchOnTrustStateRegression,
        Self::BlockProtectedDispatchOnPolicyEpochRegression,
        Self::BlockOnTargetUnreachable,
        Self::BlockOnAdapterNegotiationRefused,
    ];

    /// Stable token recorded in records, schemas, and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowLaunchOnAdapterCapabilityDrop => "narrow_launch_on_adapter_capability_drop",
            Self::NarrowAttachToInspectOnlyOnCapabilityDrop => {
                "narrow_attach_to_inspect_only_on_capability_drop"
            }
            Self::BlockOnUnclaimedTestFramework => "block_on_unclaimed_test_framework",
            Self::BlockOnUnclaimedTargetClass => "block_on_unclaimed_target_class",
            Self::BlockProtectedDispatchOnTicketDrift => {
                "block_protected_dispatch_on_ticket_drift"
            }
            Self::BlockProtectedDispatchOnCapsuleDrift => {
                "block_protected_dispatch_on_capsule_drift"
            }
            Self::BlockProtectedDispatchOnTrustStateRegression => {
                "block_protected_dispatch_on_trust_state_regression"
            }
            Self::BlockProtectedDispatchOnPolicyEpochRegression => {
                "block_protected_dispatch_on_policy_epoch_regression"
            }
            Self::BlockOnTargetUnreachable => "block_on_target_unreachable",
            Self::BlockOnAdapterNegotiationRefused => "block_on_adapter_negotiation_refused",
        }
    }

    /// Resolves the canonical enum variant from its stable token.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|rule| rule.as_str() == token)
    }
}

/// Launch column of one wedge row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixLaunchSupport {
    /// Overall support class for launch on this wedge.
    pub class: SupportMatrixClass,
    /// Stable token for the class.
    pub class_token: String,
    /// Adapter protocol class the wedge launches through. Closed vocabulary
    /// shared with the DAP host beta supervisor.
    pub adapter_protocol_token: String,
    /// Short reviewer-facing summary.
    pub summary: String,
}

/// Attach column of one wedge row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixAttachSupport {
    /// Overall support class for attach on this wedge.
    pub class: SupportMatrixClass,
    /// Stable token for the class.
    pub class_token: String,
    /// Adapter protocol class the wedge attaches through.
    pub adapter_protocol_token: String,
    /// Short reviewer-facing summary.
    pub summary: String,
}

/// Test column of one wedge row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixTestSupport {
    /// Overall support class for test on this wedge.
    pub class: SupportMatrixClass,
    /// Stable token for the class.
    pub class_token: String,
    /// Test frameworks claimed by the beta program for this wedge. Empty
    /// when the wedge is `Limited` or `Unsupported` for the test lane.
    pub claimed_framework_tokens: Vec<String>,
    /// Test frameworks visible to the user but not claimed; the row MUST
    /// render these as `Limited` rather than `Supported`.
    pub previewed_framework_tokens: Vec<String>,
    /// Short reviewer-facing summary.
    pub summary: String,
}

/// One execution-context lane row inside the execution-context column.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixContextLaneSupport {
    /// Execution-context lane.
    pub lane: SupportMatrixContextLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Support class for this lane on this wedge.
    pub class: SupportMatrixClass,
    /// Stable token for the class.
    pub class_token: String,
    /// Short reviewer-facing summary.
    pub summary: String,
}

/// Execution-context column of one wedge row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixContextSupport {
    /// Overall rollup support class for the execution-context column.
    pub overall_class: SupportMatrixClass,
    /// Stable rollup token.
    pub overall_class_token: String,
    /// Per-lane support rows.
    pub lanes: Vec<SupportMatrixContextLaneSupport>,
}

/// One wedge row in the support matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixWedgeRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Canonical wedge identifier.
    pub wedge_id: SupportMatrixWedgeId,
    /// Stable wedge token.
    pub wedge_token: String,
    /// Reviewer-facing label.
    pub wedge_label: String,
    /// Launch column.
    pub launch: SupportMatrixLaunchSupport,
    /// Attach column.
    pub attach: SupportMatrixAttachSupport,
    /// Test column.
    pub test: SupportMatrixTestSupport,
    /// Execution-context column.
    pub execution_context: SupportMatrixContextSupport,
    /// Downgrade rules that apply when context, ticket, capsule, trust, or
    /// adapter state disagrees with the stored binding.
    pub downgrade_rules: Vec<SupportMatrixDowngradeRule>,
    /// Stable downgrade-rule tokens.
    pub downgrade_rule_tokens: Vec<String>,
}

impl SupportMatrixWedgeRow {
    /// Builds the canonical row for one wedge.
    pub fn canonical(wedge_id: SupportMatrixWedgeId) -> Self {
        match wedge_id {
            SupportMatrixWedgeId::Python => canonical_python_row(),
            SupportMatrixWedgeId::TypescriptJavascript => canonical_tsjs_row(),
        }
    }

    /// True when every column on the row claims `Supported` so the wedge can
    /// dispatch protected run / test / debug work without review on a fresh,
    /// in-sync context.
    pub fn allows_protected_dispatch(&self) -> bool {
        self.launch.class.allows_protected_dispatch()
            && self.attach.class.allows_protected_dispatch()
            && self.test.class.allows_protected_dispatch()
            && self.execution_context.overall_class.allows_protected_dispatch()
    }
}

fn make_launch(
    class: SupportMatrixClass,
    adapter_protocol_token: &str,
    summary: &str,
) -> SupportMatrixLaunchSupport {
    SupportMatrixLaunchSupport {
        class,
        class_token: class.as_str().to_owned(),
        adapter_protocol_token: adapter_protocol_token.to_owned(),
        summary: summary.to_owned(),
    }
}

fn make_attach(
    class: SupportMatrixClass,
    adapter_protocol_token: &str,
    summary: &str,
) -> SupportMatrixAttachSupport {
    SupportMatrixAttachSupport {
        class,
        class_token: class.as_str().to_owned(),
        adapter_protocol_token: adapter_protocol_token.to_owned(),
        summary: summary.to_owned(),
    }
}

fn make_test(
    class: SupportMatrixClass,
    claimed: &[&str],
    previewed: &[&str],
    summary: &str,
) -> SupportMatrixTestSupport {
    SupportMatrixTestSupport {
        class,
        class_token: class.as_str().to_owned(),
        claimed_framework_tokens: claimed.iter().map(|s| (*s).to_owned()).collect(),
        previewed_framework_tokens: previewed.iter().map(|s| (*s).to_owned()).collect(),
        summary: summary.to_owned(),
    }
}

fn make_lane(
    lane: SupportMatrixContextLane,
    class: SupportMatrixClass,
    summary: &str,
) -> SupportMatrixContextLaneSupport {
    SupportMatrixContextLaneSupport {
        lane,
        lane_token: lane.as_str().to_owned(),
        class,
        class_token: class.as_str().to_owned(),
        summary: summary.to_owned(),
    }
}

fn rollup_class(rows: &[SupportMatrixContextLaneSupport]) -> SupportMatrixClass {
    // The rollup is the weakest non-unsupported class. If every lane is
    // unsupported the rollup is unsupported. Preview beats Limited because
    // Preview lanes are wired but unexercised, while Limited lanes are
    // narrowly available but cannot dispatch protected work.
    let mut rollup = SupportMatrixClass::Supported;
    let mut any_supported = false;
    for row in rows {
        match row.class {
            SupportMatrixClass::Supported => any_supported = true,
            SupportMatrixClass::Preview => {
                if matches!(rollup, SupportMatrixClass::Supported) {
                    rollup = SupportMatrixClass::Preview;
                }
            }
            SupportMatrixClass::Limited => {
                if matches!(rollup, SupportMatrixClass::Supported | SupportMatrixClass::Preview)
                {
                    rollup = SupportMatrixClass::Limited;
                }
            }
            SupportMatrixClass::Unsupported => {
                if matches!(
                    rollup,
                    SupportMatrixClass::Supported
                        | SupportMatrixClass::Preview
                        | SupportMatrixClass::Limited
                ) {
                    rollup = SupportMatrixClass::Unsupported;
                }
            }
        }
    }
    if any_supported && matches!(rollup, SupportMatrixClass::Unsupported) {
        // If at least one lane is supported but another is unsupported,
        // surface as Limited rather than Unsupported.
        SupportMatrixClass::Limited
    } else {
        rollup
    }
}

fn make_context(rows: Vec<SupportMatrixContextLaneSupport>) -> SupportMatrixContextSupport {
    let overall_class = rollup_class(&rows);
    SupportMatrixContextSupport {
        overall_class,
        overall_class_token: overall_class.as_str().to_owned(),
        lanes: rows,
    }
}

fn make_row(
    wedge_id: SupportMatrixWedgeId,
    launch: SupportMatrixLaunchSupport,
    attach: SupportMatrixAttachSupport,
    test: SupportMatrixTestSupport,
    execution_context: SupportMatrixContextSupport,
    rules: &[SupportMatrixDowngradeRule],
) -> SupportMatrixWedgeRow {
    let downgrade_rule_tokens = rules.iter().map(|rule| rule.as_str().to_owned()).collect();
    SupportMatrixWedgeRow {
        record_kind: SUPPORT_MATRIX_BETA_WEDGE_ROW_RECORD_KIND.to_owned(),
        schema_version: SUPPORT_MATRIX_BETA_SCHEMA_VERSION,
        wedge_id,
        wedge_token: wedge_id.as_str().to_owned(),
        wedge_label: wedge_id.label().to_owned(),
        launch,
        attach,
        test,
        execution_context,
        downgrade_rules: rules.to_vec(),
        downgrade_rule_tokens,
    }
}

fn canonical_python_row() -> SupportMatrixWedgeRow {
    let launch = make_launch(
        SupportMatrixClass::Supported,
        "dap_helper",
        "Python debug-launch is wired through the beta DAP host with adapter \
         capability negotiation; launches that drop a requested capability \
         record the drop on the session snapshot.",
    );
    let attach = make_attach(
        SupportMatrixClass::Preview,
        "dap_helper",
        "Attach is reachable through the same DAP host but is not exercised \
         by claimed beta workflows; rows render but protected attach \
         dispatch requires review.",
    );
    let test = make_test(
        SupportMatrixClass::Supported,
        &["pytest"],
        &[],
        "Pytest discovery, run, rerun, tree, inline marker, and artifact \
         identity are part of the claimed coverage manifest; other Python \
         test frameworks fall outside the manifest and render as unclaimed.",
    );
    let context = make_context(vec![
        make_lane(
            SupportMatrixContextLane::LocalHost,
            SupportMatrixClass::Supported,
            "Local host terminal, task, test, debug, and ai_tool_call \
             surfaces resolve through the local_host lane.",
        ),
        make_lane(
            SupportMatrixContextLane::Container,
            SupportMatrixClass::Supported,
            "Devcontainer / container lane resolves the same execution \
             context the local host lane carries, with the boundary cue \
             visible.",
        ),
        make_lane(
            SupportMatrixContextLane::RemoteAttach,
            SupportMatrixClass::Preview,
            "Remote-attach lane is wired and inspectable but Python remote \
             debug attach is not exercised by claimed beta workflows.",
        ),
        make_lane(
            SupportMatrixContextLane::RequestWorkspace,
            SupportMatrixClass::Preview,
            "Request-workspace lane resolves managed-workspace seeds; \
             protected dispatch requires review per the trust contract.",
        ),
    ]);
    let rules = [
        SupportMatrixDowngradeRule::NarrowLaunchOnAdapterCapabilityDrop,
        SupportMatrixDowngradeRule::NarrowAttachToInspectOnlyOnCapabilityDrop,
        SupportMatrixDowngradeRule::BlockOnUnclaimedTestFramework,
        SupportMatrixDowngradeRule::BlockOnUnclaimedTargetClass,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnTicketDrift,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnCapsuleDrift,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnTrustStateRegression,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnPolicyEpochRegression,
        SupportMatrixDowngradeRule::BlockOnTargetUnreachable,
        SupportMatrixDowngradeRule::BlockOnAdapterNegotiationRefused,
    ];
    make_row(
        SupportMatrixWedgeId::Python,
        launch,
        attach,
        test,
        context,
        &rules,
    )
}

fn canonical_tsjs_row() -> SupportMatrixWedgeRow {
    let launch = make_launch(
        SupportMatrixClass::Preview,
        "dap_helper",
        "TS/JS debug-launch is reachable through the DAP host beta but is \
         not exercised by claimed test or debug coverage; rows render but \
         protected launch dispatch requires review.",
    );
    let attach = make_attach(
        SupportMatrixClass::Preview,
        "dap_helper",
        "Attach is reachable through the same DAP host; like launch, it is \
         not exercised by claimed coverage and requires review before \
         protected dispatch.",
    );
    let test = make_test(
        SupportMatrixClass::Limited,
        &[],
        &["jest", "vitest", "node_test"],
        "No TS/JS framework is on the claimed beta coverage manifest yet; \
         rows render as unclaimed and the rerun-last command reports \
         unavailable until a framework joins the manifest.",
    );
    let context = make_context(vec![
        make_lane(
            SupportMatrixContextLane::LocalHost,
            SupportMatrixClass::Supported,
            "Local host terminal, task, and execution-context resolution is \
             the claimed daily-driver lane for TS/JS workflows.",
        ),
        make_lane(
            SupportMatrixContextLane::Container,
            SupportMatrixClass::Preview,
            "Devcontainer / container lane resolves the same execution \
             context but is not exercised by claimed coverage.",
        ),
        make_lane(
            SupportMatrixContextLane::RemoteAttach,
            SupportMatrixClass::Limited,
            "Remote-attach lane is reachable for inspection only; \
             protected TS/JS dispatch fails closed.",
        ),
        make_lane(
            SupportMatrixContextLane::RequestWorkspace,
            SupportMatrixClass::Limited,
            "Request-workspace lane resolves managed-workspace seeds but \
             cannot dispatch protected TS/JS work without a claimed \
             framework.",
        ),
    ]);
    let rules = [
        SupportMatrixDowngradeRule::NarrowLaunchOnAdapterCapabilityDrop,
        SupportMatrixDowngradeRule::NarrowAttachToInspectOnlyOnCapabilityDrop,
        SupportMatrixDowngradeRule::BlockOnUnclaimedTestFramework,
        SupportMatrixDowngradeRule::BlockOnUnclaimedTargetClass,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnTicketDrift,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnCapsuleDrift,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnTrustStateRegression,
        SupportMatrixDowngradeRule::BlockProtectedDispatchOnPolicyEpochRegression,
        SupportMatrixDowngradeRule::BlockOnTargetUnreachable,
        SupportMatrixDowngradeRule::BlockOnAdapterNegotiationRefused,
    ];
    make_row(
        SupportMatrixWedgeId::TypescriptJavascript,
        launch,
        attach,
        test,
        context,
        &rules,
    )
}

/// Canonical manifest pinning the support matrix the beta program ships.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixBetaManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Canonical wedge rows in iteration order.
    pub rows: Vec<SupportMatrixWedgeRow>,
}

impl SupportMatrixBetaManifest {
    /// Builds the canonical manifest with the canonical wedge rows.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        Self {
            record_kind: SUPPORT_MATRIX_BETA_MANIFEST_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_MATRIX_BETA_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            rows: SupportMatrixWedgeId::ALL
                .into_iter()
                .map(SupportMatrixWedgeRow::canonical)
                .collect(),
        }
    }

    /// Returns the canonical row for one wedge, when present.
    pub fn row_for_wedge(&self, wedge_id: SupportMatrixWedgeId) -> Option<&SupportMatrixWedgeRow> {
        self.rows.iter().find(|row| row.wedge_id == wedge_id)
    }
}

/// One partner / migration / release input row loaded from a fixture file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixWedgeInput {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Canonical wedge id.
    pub wedge_id: SupportMatrixWedgeId,
    /// Stable wedge token.
    pub wedge_token: String,
    /// Expected class for the launch column.
    pub expected_launch_class: SupportMatrixClass,
    /// Expected class for the attach column.
    pub expected_attach_class: SupportMatrixClass,
    /// Expected class for the test column.
    pub expected_test_class: SupportMatrixClass,
    /// Expected claimed test frameworks.
    pub expected_claimed_framework_tokens: Vec<String>,
    /// Expected per-lane execution-context support.
    pub expected_context_lanes: Vec<SupportMatrixContextLaneExpectation>,
    /// Expected downgrade rules.
    pub expected_downgrade_rule_tokens: Vec<String>,
}

/// One expected per-lane execution-context class on a wedge input fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixContextLaneExpectation {
    /// Lane token (`local_host`, `remote_attach`, `container`,
    /// `request_workspace`).
    pub lane_token: String,
    /// Expected support class for the lane.
    pub expected_class: SupportMatrixClass,
}

/// Closed vocabulary of mismatch reasons emitted by
/// [`SupportMatrixBetaManifest::compare_input`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum SupportMatrixInputMismatch {
    /// The wedge in the input fixture is not in the canonical manifest.
    UnknownWedge,
    /// Launch column disagrees between input and canonical row.
    LaunchClassMismatch {
        expected_token: String,
        canonical_token: String,
    },
    /// Attach column disagrees.
    AttachClassMismatch {
        expected_token: String,
        canonical_token: String,
    },
    /// Test column disagrees.
    TestClassMismatch {
        expected_token: String,
        canonical_token: String,
    },
    /// Claimed test framework set disagrees.
    ClaimedTestFrameworksMismatch {
        expected: Vec<String>,
        canonical: Vec<String>,
    },
    /// One execution-context lane class disagrees.
    ContextLaneClassMismatch {
        lane_token: String,
        expected_token: String,
        canonical_token: String,
    },
    /// One execution-context lane on the input was not in the canonical
    /// lane vocabulary.
    UnknownContextLane { lane_token: String },
    /// Downgrade-rule set disagrees.
    DowngradeRulesMismatch {
        expected: Vec<String>,
        canonical: Vec<String>,
    },
}

impl SupportMatrixBetaManifest {
    /// Compares an input fixture row against the canonical row for the same
    /// wedge, returning the full mismatch list (empty when the fixture
    /// matches the canonical row).
    pub fn compare_input(&self, input: &SupportMatrixWedgeInput) -> Vec<SupportMatrixInputMismatch> {
        let canonical = match self.row_for_wedge(input.wedge_id) {
            Some(row) => row,
            None => return vec![SupportMatrixInputMismatch::UnknownWedge],
        };
        let mut mismatches = Vec::new();
        if input.expected_launch_class != canonical.launch.class {
            mismatches.push(SupportMatrixInputMismatch::LaunchClassMismatch {
                expected_token: input.expected_launch_class.as_str().to_owned(),
                canonical_token: canonical.launch.class_token.clone(),
            });
        }
        if input.expected_attach_class != canonical.attach.class {
            mismatches.push(SupportMatrixInputMismatch::AttachClassMismatch {
                expected_token: input.expected_attach_class.as_str().to_owned(),
                canonical_token: canonical.attach.class_token.clone(),
            });
        }
        if input.expected_test_class != canonical.test.class {
            mismatches.push(SupportMatrixInputMismatch::TestClassMismatch {
                expected_token: input.expected_test_class.as_str().to_owned(),
                canonical_token: canonical.test.class_token.clone(),
            });
        }
        if input.expected_claimed_framework_tokens != canonical.test.claimed_framework_tokens {
            mismatches.push(SupportMatrixInputMismatch::ClaimedTestFrameworksMismatch {
                expected: input.expected_claimed_framework_tokens.clone(),
                canonical: canonical.test.claimed_framework_tokens.clone(),
            });
        }
        for lane in &input.expected_context_lanes {
            let canonical_lane = canonical
                .execution_context
                .lanes
                .iter()
                .find(|row| row.lane_token == lane.lane_token);
            match canonical_lane {
                None => mismatches.push(SupportMatrixInputMismatch::UnknownContextLane {
                    lane_token: lane.lane_token.clone(),
                }),
                Some(canonical_lane) => {
                    if lane.expected_class != canonical_lane.class {
                        mismatches.push(SupportMatrixInputMismatch::ContextLaneClassMismatch {
                            lane_token: lane.lane_token.clone(),
                            expected_token: lane.expected_class.as_str().to_owned(),
                            canonical_token: canonical_lane.class_token.clone(),
                        });
                    }
                }
            }
        }
        if input.expected_downgrade_rule_tokens != canonical.downgrade_rule_tokens {
            mismatches.push(SupportMatrixInputMismatch::DowngradeRulesMismatch {
                expected: input.expected_downgrade_rule_tokens.clone(),
                canonical: canonical.downgrade_rule_tokens.clone(),
            });
        }
        mismatches
    }
}

/// Support-export packet shipped with migration / partner / release bundles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id minted by the caller.
    pub support_export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Canonical manifest the export bundles.
    pub manifest: SupportMatrixBetaManifest,
    /// Resolved input rows the caller wants to ship alongside the manifest.
    pub inputs: Vec<SupportMatrixWedgeInput>,
}

impl SupportMatrixBetaSupportExport {
    /// Builds the support-export bundle from a manifest and input rows.
    pub fn new(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        manifest: SupportMatrixBetaManifest,
        inputs: Vec<SupportMatrixWedgeInput>,
    ) -> Self {
        Self {
            record_kind: SUPPORT_MATRIX_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_MATRIX_BETA_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            manifest,
            inputs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_manifest_has_one_row_per_claimed_wedge() {
        let manifest =
            SupportMatrixBetaManifest::canonical("matrix:test", "2026-05-15T00:00:00Z");
        assert_eq!(manifest.rows.len(), SupportMatrixWedgeId::ALL.len());
        for wedge_id in SupportMatrixWedgeId::ALL {
            let row = manifest
                .row_for_wedge(wedge_id)
                .unwrap_or_else(|| panic!("missing row for {wedge_id:?}"));
            assert_eq!(row.wedge_token, wedge_id.as_str());
            assert_eq!(row.wedge_label, wedge_id.label());
            assert_eq!(
                row.execution_context.lanes.len(),
                SupportMatrixContextLane::ALL.len(),
                "wedge {wedge_id:?} must declare every canonical lane"
            );
        }
    }

    #[test]
    fn python_row_claims_pytest_and_local_host_lane() {
        let row = SupportMatrixWedgeRow::canonical(SupportMatrixWedgeId::Python);
        assert_eq!(row.launch.class_token, "supported");
        assert_eq!(row.attach.class_token, "preview");
        assert_eq!(row.test.class_token, "supported");
        assert_eq!(row.test.claimed_framework_tokens, vec!["pytest".to_owned()]);
        let local = row
            .execution_context
            .lanes
            .iter()
            .find(|lane| lane.lane_token == "local_host")
            .expect("local_host lane");
        assert_eq!(local.class_token, "supported");
    }

    #[test]
    fn tsjs_row_is_preview_and_test_is_limited() {
        let row = SupportMatrixWedgeRow::canonical(SupportMatrixWedgeId::TypescriptJavascript);
        assert_eq!(row.launch.class_token, "preview");
        assert_eq!(row.attach.class_token, "preview");
        assert_eq!(row.test.class_token, "limited");
        assert!(row.test.claimed_framework_tokens.is_empty());
        assert!(!row.allows_protected_dispatch());
    }

    #[test]
    fn rollup_class_demotes_when_any_lane_is_preview() {
        let lanes = vec![
            make_lane(
                SupportMatrixContextLane::LocalHost,
                SupportMatrixClass::Supported,
                "",
            ),
            make_lane(
                SupportMatrixContextLane::Container,
                SupportMatrixClass::Preview,
                "",
            ),
        ];
        assert_eq!(rollup_class(&lanes), SupportMatrixClass::Preview);
    }

    #[test]
    fn rollup_class_collapses_mixed_supported_and_unsupported_to_limited() {
        let lanes = vec![
            make_lane(
                SupportMatrixContextLane::LocalHost,
                SupportMatrixClass::Supported,
                "",
            ),
            make_lane(
                SupportMatrixContextLane::RemoteAttach,
                SupportMatrixClass::Unsupported,
                "",
            ),
        ];
        assert_eq!(rollup_class(&lanes), SupportMatrixClass::Limited);
    }

    #[test]
    fn input_comparison_reports_class_mismatch() {
        let manifest =
            SupportMatrixBetaManifest::canonical("matrix:test", "2026-05-15T00:00:00Z");
        let input = SupportMatrixWedgeInput {
            record_kind: SUPPORT_MATRIX_BETA_WEDGE_INPUT_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_MATRIX_BETA_SCHEMA_VERSION,
            wedge_id: SupportMatrixWedgeId::Python,
            wedge_token: "python".to_owned(),
            expected_launch_class: SupportMatrixClass::Preview,
            expected_attach_class: SupportMatrixClass::Preview,
            expected_test_class: SupportMatrixClass::Supported,
            expected_claimed_framework_tokens: vec!["pytest".to_owned()],
            expected_context_lanes: vec![SupportMatrixContextLaneExpectation {
                lane_token: "local_host".to_owned(),
                expected_class: SupportMatrixClass::Supported,
            }],
            expected_downgrade_rule_tokens: manifest
                .row_for_wedge(SupportMatrixWedgeId::Python)
                .unwrap()
                .downgrade_rule_tokens
                .clone(),
        };
        let mismatches = manifest.compare_input(&input);
        assert_eq!(mismatches.len(), 1);
        assert!(matches!(
            mismatches[0],
            SupportMatrixInputMismatch::LaunchClassMismatch { .. }
        ));
    }

    #[test]
    fn downgrade_rule_tokens_roundtrip_through_from_token() {
        for rule in SupportMatrixDowngradeRule::ALL {
            let token = rule.as_str();
            assert_eq!(SupportMatrixDowngradeRule::from_token(token), Some(rule));
        }
        assert_eq!(SupportMatrixDowngradeRule::from_token("nope"), None);
    }
}
