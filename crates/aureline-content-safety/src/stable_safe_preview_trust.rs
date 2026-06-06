//! Stable safe-preview trust-class contract and consumer matrix.
//!
//! This module turns the four safe-preview trust classes into a single
//! machine-readable packet that editor, docs/help preview, notebook rich output,
//! preview/runtime, embedded marketplace/account, browser-runtime, and
//! support/export surfaces can consume without inventing local copy/export or
//! downgrade rules.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{RepresentationActionId, RepresentationClass, TrustClass};

/// Schema version for stable safe-preview trust packets.
pub const STABLE_SAFE_PREVIEW_TRUST_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`StableSafePreviewTrustPacket`].
pub const STABLE_SAFE_PREVIEW_TRUST_PACKET_RECORD_KIND: &str = "stable_safe_preview_trust_packet";

/// Stable record-kind tag for [`StableSafePreviewTrustValidationReport`].
pub const STABLE_SAFE_PREVIEW_TRUST_VALIDATION_REPORT_RECORD_KIND: &str =
    "stable_safe_preview_trust_validation_report";

/// Repo-relative machine-readable schema for this packet.
pub const STABLE_SAFE_PREVIEW_TRUST_SCHEMA_REF: &str =
    "schemas/trust/safe-preview-trust-class.schema.json";

/// Repo-relative human-readable trust contract.
pub const STABLE_SAFE_PREVIEW_TRUST_DOC_REF: &str =
    "docs/trust/m4/stabilize-safe-preview-trust-classes.md";

/// Repo-relative fixture directory.
pub const STABLE_SAFE_PREVIEW_TRUST_FIXTURE_DIR: &str =
    "fixtures/trust/m4/stabilize-safe-preview-trust-classes";

/// Shared contract ref consumed by stable surface rows.
pub const STABLE_SAFE_PREVIEW_SHARED_CONTRACT_REF: &str =
    "content-safety:stable_safe_preview_trust:v1";

/// Stable consumer surfaces required by the safe-preview trust matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewConsumerSurface {
    /// Plain editor or source text surface.
    Editor,
    /// In-product docs or help preview.
    DocsHelpPreview,
    /// Notebook rich-output block.
    NotebookRichOutput,
    /// Product preview/runtime pane.
    PreviewRuntime,
    /// Marketplace or account embedded webview.
    MarketplaceAccountWebview,
    /// Browser-runtime viewer or inspector.
    BrowserRuntimeViewer,
    /// Support bundle, screenshot, diagnostics, or exported evidence surface.
    SupportExport,
    /// Extension, package, or plugin install review.
    InstallReview,
    /// Remote attach or route approval review.
    AttachReview,
    /// Native approval surface.
    ApprovalReview,
    /// Publish review surface.
    PublishReview,
    /// Delete review surface preserving last-visible evidence.
    DeleteReview,
}

impl SafePreviewConsumerSurface {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::DocsHelpPreview => "docs_help_preview",
            Self::NotebookRichOutput => "notebook_rich_output",
            Self::PreviewRuntime => "preview_runtime",
            Self::MarketplaceAccountWebview => "marketplace_account_webview",
            Self::BrowserRuntimeViewer => "browser_runtime_viewer",
            Self::SupportExport => "support_export",
            Self::InstallReview => "install_review",
            Self::AttachReview => "attach_review",
            Self::ApprovalReview => "approval_review",
            Self::PublishReview => "publish_review",
            Self::DeleteReview => "delete_review",
        }
    }

    /// Returns true for surfaces that decide trust-sensitive actions.
    pub const fn is_decision_surface(self) -> bool {
        matches!(
            self,
            Self::InstallReview
                | Self::AttachReview
                | Self::ApprovalReview
                | Self::PublishReview
                | Self::DeleteReview
        )
    }
}

/// Stable product surfaces that must consume this packet.
pub const REQUIRED_STABLE_CONSUMER_SURFACES: [SafePreviewConsumerSurface; 12] = [
    SafePreviewConsumerSurface::Editor,
    SafePreviewConsumerSurface::DocsHelpPreview,
    SafePreviewConsumerSurface::NotebookRichOutput,
    SafePreviewConsumerSurface::PreviewRuntime,
    SafePreviewConsumerSurface::MarketplaceAccountWebview,
    SafePreviewConsumerSurface::BrowserRuntimeViewer,
    SafePreviewConsumerSurface::SupportExport,
    SafePreviewConsumerSurface::InstallReview,
    SafePreviewConsumerSurface::AttachReview,
    SafePreviewConsumerSurface::ApprovalReview,
    SafePreviewConsumerSurface::PublishReview,
    SafePreviewConsumerSurface::DeleteReview,
];

/// Surface claim level after the trust-class validator runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceQualification {
    /// The row satisfies every stable trust-class rule.
    Stable,
    /// The row is present only as a lower-lifecycle proof or drill.
    BelowStable,
}

impl SurfaceQualification {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::BelowStable => "below_stable",
        }
    }
}

/// Rendering or transfer behavior admitted by a trust class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewAllowedBehavior {
    /// Render exact source bytes or exact source text.
    RenderExactBytes,
    /// Render sanitized markup or structured rich content.
    RenderSanitizedMarkup,
    /// Run active local content inside a declared local capability sandbox.
    RunLocalActiveContent,
    /// Run remote or embedded active content inside declared isolation.
    RunIsolatedRemoteActiveContent,
    /// Show inline warning overlays.
    ShowInlineWarningOverlays,
    /// Reveal source in raw form.
    RevealRawSource,
    /// Reveal source in escaped form.
    RevealEscapedSource,
    /// Open a static snapshot when active guarantees are lost.
    OpenStaticSnapshot,
    /// Block active behavior while keeping reviewable evidence visible.
    BlockActiveContent,
}

impl SafePreviewAllowedBehavior {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenderExactBytes => "render_exact_bytes",
            Self::RenderSanitizedMarkup => "render_sanitized_markup",
            Self::RunLocalActiveContent => "run_local_active_content",
            Self::RunIsolatedRemoteActiveContent => "run_isolated_remote_active_content",
            Self::ShowInlineWarningOverlays => "show_inline_warning_overlays",
            Self::RevealRawSource => "reveal_raw_source",
            Self::RevealEscapedSource => "reveal_escaped_source",
            Self::OpenStaticSnapshot => "open_static_snapshot",
            Self::BlockActiveContent => "block_active_content",
        }
    }
}

/// Required visible cue carried by a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibleTrustCue {
    /// Trust-class badge is visible.
    TrustClassBadge,
    /// Representation label is visible on the surface and transfer actions.
    RepresentationLabel,
    /// Raw-view path is visible when relevant.
    RawViewPath,
    /// Owner identity is visible.
    OwnerIdentity,
    /// Origin or host boundary is visible.
    OriginBoundary,
    /// Capability summary is visible.
    CapabilitySummary,
    /// Permission summary is visible.
    PermissionSummary,
    /// Downgrade explanation is visible.
    DowngradeExplanation,
}

impl VisibleTrustCue {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustClassBadge => "trust_class_badge",
            Self::RepresentationLabel => "representation_label",
            Self::RawViewPath => "raw_view_path",
            Self::OwnerIdentity => "owner_identity",
            Self::OriginBoundary => "origin_boundary",
            Self::CapabilitySummary => "capability_summary",
            Self::PermissionSummary => "permission_summary",
            Self::DowngradeExplanation => "downgrade_explanation",
        }
    }
}

/// Trigger that can narrow a safe-preview surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewDowngradeTrigger {
    /// Workspace or item trust was revoked.
    TrustLoss,
    /// Policy denied or narrowed the active path.
    PolicyDeny,
    /// Remote or embedded connectivity was lost.
    Disconnect,
    /// Owner, origin, or route identity was lost.
    OriginLoss,
    /// Host or renderer is unsupported.
    UnsupportedHost,
    /// Script, widget, or active capability was blocked.
    BlockedActiveCapability,
    /// Support/export boundary forbids carrying active content.
    SupportExportBoundary,
}

impl SafePreviewDowngradeTrigger {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustLoss => "trust_loss",
            Self::PolicyDeny => "policy_deny",
            Self::Disconnect => "disconnect",
            Self::OriginLoss => "origin_loss",
            Self::UnsupportedHost => "unsupported_host",
            Self::BlockedActiveCapability => "blocked_active_capability",
            Self::SupportExportBoundary => "support_export_boundary",
        }
    }
}

/// Effective state after downgrade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewDowngradeState {
    /// No downgrade is active.
    None,
    /// Surface narrowed to sanitized rich content.
    Sanitized,
    /// Surface narrowed to a static snapshot.
    StaticSnapshot,
    /// Surface narrowed to metadata only.
    MetadataOnly,
    /// Surface blocks active body rendering.
    Blocked,
}

impl SafePreviewDowngradeState {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Sanitized => "sanitized",
            Self::StaticSnapshot => "static_snapshot",
            Self::MetadataOnly => "metadata_only",
            Self::Blocked => "blocked",
        }
    }
}

/// Origin boundary visible on a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginBoundaryClass {
    /// Local workspace or local file.
    LocalWorkspace,
    /// Local docs pack or mirrored help source.
    LocalDocsPack,
    /// Local trusted runtime.
    TrustedLocalRuntime,
    /// Remote route or provider origin.
    RemoteProviderOrigin,
    /// Embedded marketplace/account origin.
    EmbeddedMarketplaceOrigin,
    /// Browser-runtime inspected origin.
    BrowserRuntimeOrigin,
    /// Support/export snapshot origin.
    SupportExportSnapshot,
}

impl OriginBoundaryClass {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::LocalDocsPack => "local_docs_pack",
            Self::TrustedLocalRuntime => "trusted_local_runtime",
            Self::RemoteProviderOrigin => "remote_provider_origin",
            Self::EmbeddedMarketplaceOrigin => "embedded_marketplace_origin",
            Self::BrowserRuntimeOrigin => "browser_runtime_origin",
            Self::SupportExportSnapshot => "support_export_snapshot",
        }
    }
}

/// Carrier that must preserve trust truth outside the live UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustEvidenceCarrier {
    /// Screenshot or visual evidence capture.
    Screenshot,
    /// Support bundle.
    SupportBundle,
    /// Exported evidence packet.
    ExportedEvidence,
    /// Diagnostic record.
    Diagnostics,
}

impl TrustEvidenceCarrier {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Screenshot => "screenshot",
            Self::SupportBundle => "support_bundle",
            Self::ExportedEvidence => "exported_evidence",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Required evidence carriers for stable rows.
pub const REQUIRED_TRUST_EVIDENCE_CARRIERS: [TrustEvidenceCarrier; 4] = [
    TrustEvidenceCarrier::Screenshot,
    TrustEvidenceCarrier::SupportBundle,
    TrustEvidenceCarrier::ExportedEvidence,
    TrustEvidenceCarrier::Diagnostics,
];

/// Class of copy/export case covered by the corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewTransferCaseKind {
    /// Raw-only content.
    RawOnly,
    /// Sanitized rich content.
    Sanitized,
    /// Trusted local active content.
    TrustedLocal,
    /// Isolated remote active content.
    IsolatedRemote,
    /// Active content downgraded after guarantee loss.
    Downgrade,
    /// Blocked body or metadata-only case.
    Blocked,
}

impl SafePreviewTransferCaseKind {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawOnly => "raw_only",
            Self::Sanitized => "sanitized",
            Self::TrustedLocal => "trusted_local",
            Self::IsolatedRemote => "isolated_remote",
            Self::Downgrade => "downgrade",
            Self::Blocked => "blocked",
        }
    }
}

/// Required copy/export case coverage.
pub const REQUIRED_TRANSFER_CASE_KINDS: [SafePreviewTransferCaseKind; 6] = [
    SafePreviewTransferCaseKind::RawOnly,
    SafePreviewTransferCaseKind::Sanitized,
    SafePreviewTransferCaseKind::TrustedLocal,
    SafePreviewTransferCaseKind::IsolatedRemote,
    SafePreviewTransferCaseKind::Downgrade,
    SafePreviewTransferCaseKind::Blocked,
];

/// Canonical contract for one trust class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewTrustClassContract {
    /// Trust class governed by this contract row.
    pub trust_class: TrustClass,
    /// Behaviors admitted while the class is in force.
    pub allowed_behaviors: Vec<SafePreviewAllowedBehavior>,
    /// Required cues for surfaces in this class.
    pub visible_cues: Vec<VisibleTrustCue>,
    /// Default copy/export transfer actions.
    pub default_transfer_actions: Vec<RepresentationActionId>,
    /// Upgrade requirements, expressed as stable tokens.
    pub upgrade_requirements: Vec<String>,
    /// Downgrade triggers that must be handled visibly.
    pub downgrade_triggers: Vec<SafePreviewDowngradeTrigger>,
    /// State the surface must fall to when guarantees are lost.
    pub fallback_states: Vec<SafePreviewDowngradeState>,
}

/// Trust truth projected by one stable consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewSurfaceMatrixRow {
    /// Stable row id.
    pub row_id: String,
    /// Consumer surface covered by this row.
    pub surface: SafePreviewConsumerSurface,
    /// Effective release qualification.
    pub qualification: SurfaceQualification,
    /// Shared contract ref consumed by the row.
    pub consumed_contract_ref: String,
    /// Current trust class.
    pub trust_class: TrustClass,
    /// Representation currently visible.
    pub visible_representation: RepresentationClass,
    /// True when raw and rendered meaning can differ materially.
    pub raw_rendered_distinction_meaningful: bool,
    /// True when the source representation is available to this surface.
    pub source_representation_available: bool,
    /// True when the raw view path is visible where relevant.
    pub raw_view_path_visible: bool,
    /// Origin or host boundary class.
    pub origin_boundary: OriginBoundaryClass,
    /// True when owner identity is visible.
    pub owner_identity_visible: bool,
    /// True when origin or host identity is visible.
    pub origin_identity_visible: bool,
    /// True when trust class is visible.
    pub trust_class_visible: bool,
    /// True when representation label is visible.
    pub representation_label_visible: bool,
    /// True when capability summary is visible.
    pub capability_summary_visible: bool,
    /// True when permission summary is visible.
    pub permission_summary_visible: bool,
    /// Transfer actions available on this surface.
    pub transfer_actions: Vec<RepresentationActionId>,
    /// Downgrade triggers handled by the surface.
    pub handled_downgrade_triggers: Vec<SafePreviewDowngradeTrigger>,
    /// Effective downgrade state.
    pub effective_downgrade_state: SafePreviewDowngradeState,
    /// True when richer content finishing load cannot auto-upgrade the surface.
    pub auto_upgrade_blocked: bool,
    /// Evidence carriers preserving this row outside live UI.
    pub evidence_carriers: Vec<TrustEvidenceCarrier>,
}

/// Copy/export/review fixture row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewTransferCase {
    /// Stable case id.
    pub case_id: String,
    /// Case kind covered by this row.
    pub case_kind: SafePreviewTransferCaseKind,
    /// Surface that emits the transfer.
    pub surface: SafePreviewConsumerSurface,
    /// Source trust class.
    pub source_trust_class: TrustClass,
    /// Action being tested.
    pub action_id: RepresentationActionId,
    /// Representation leaving the surface.
    pub representation_class: RepresentationClass,
    /// Visible action label.
    pub visible_label: String,
    /// True when raw-vs-rendered choice is preserved where meaningful.
    pub preserves_raw_rendered_choice: bool,
    /// True when trust-class lineage is written to the transfer.
    pub trust_class_lineage_preserved: bool,
    /// True when origin truth is written to the transfer.
    pub origin_truth_preserved: bool,
    /// True when permission/capability summary is written where relevant.
    pub permission_truth_preserved: bool,
    /// Downgrade trigger exercised by this case.
    pub downgrade_trigger: Option<SafePreviewDowngradeTrigger>,
    /// Effective state after downgrade.
    pub effective_downgrade_state: SafePreviewDowngradeState,
    /// True when the action is safe for support/export boundaries.
    pub support_export_safe: bool,
}

/// Stable packet consumed by safe-preview surfaces and fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSafePreviewTrustPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Repo-relative schema ref.
    pub schema_ref: String,
    /// Repo-relative docs ref.
    pub doc_ref: String,
    /// Trust-class contract rows.
    pub trust_class_contracts: Vec<SafePreviewTrustClassContract>,
    /// Consumer surface matrix.
    pub surface_matrix: Vec<SafePreviewSurfaceMatrixRow>,
    /// Copy/export/review fixture cases.
    pub transfer_cases: Vec<SafePreviewTransferCase>,
}

impl StableSafePreviewTrustPacket {
    /// Validates stable safe-preview trust coverage and copy/export honesty.
    pub fn validate(&self) -> StableSafePreviewTrustValidationReport {
        let mut violations = Vec::new();

        if self.record_kind != STABLE_SAFE_PREVIEW_TRUST_PACKET_RECORD_KIND {
            violations.push(StableSafePreviewTrustViolation::InvalidRecordKind);
        }
        if self.schema_version != STABLE_SAFE_PREVIEW_TRUST_SCHEMA_VERSION {
            violations.push(StableSafePreviewTrustViolation::InvalidSchemaVersion);
        }
        if self.shared_contract_ref != STABLE_SAFE_PREVIEW_SHARED_CONTRACT_REF {
            violations.push(StableSafePreviewTrustViolation::MissingSharedContractRef);
        }

        let contract_classes = self
            .trust_class_contracts
            .iter()
            .map(|contract| contract.trust_class)
            .collect::<BTreeSet<_>>();
        for class in required_trust_classes() {
            if !contract_classes.contains(&class) {
                violations.push(StableSafePreviewTrustViolation::MissingTrustClass {
                    trust_class: class.as_str().to_string(),
                });
            }
        }
        let duplicate_contracts = duplicate_tokens(
            self.trust_class_contracts
                .iter()
                .map(|contract| contract.trust_class.as_str()),
        );
        for trust_class in duplicate_contracts {
            violations.push(StableSafePreviewTrustViolation::DuplicateTrustClass { trust_class });
        }

        let contract_by_class = self
            .trust_class_contracts
            .iter()
            .map(|contract| (contract.trust_class, contract))
            .collect::<BTreeMap<_, _>>();

        let stable_surface_counts = self
            .surface_matrix
            .iter()
            .filter(|row| row.qualification == SurfaceQualification::Stable)
            .fold(
                BTreeMap::<SafePreviewConsumerSurface, usize>::new(),
                |mut acc, row| {
                    *acc.entry(row.surface).or_default() += 1;
                    acc
                },
            );
        for surface in REQUIRED_STABLE_CONSUMER_SURFACES {
            match stable_surface_counts.get(&surface).copied().unwrap_or(0) {
                0 => violations.push(StableSafePreviewTrustViolation::MissingStableSurface {
                    surface: surface.as_str().to_string(),
                }),
                1 => {}
                _ => violations.push(StableSafePreviewTrustViolation::DuplicateStableSurface {
                    surface: surface.as_str().to_string(),
                }),
            }
        }

        for row in &self.surface_matrix {
            if row.consumed_contract_ref != self.shared_contract_ref {
                violations.push(
                    StableSafePreviewTrustViolation::SurfaceDoesNotConsumeContract {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.qualification == SurfaceQualification::Stable && !row.owner_identity_visible
                || row.qualification == SurfaceQualification::Stable && !row.origin_identity_visible
                || row.qualification == SurfaceQualification::Stable && !row.trust_class_visible
                || row.qualification == SurfaceQualification::Stable
                    && !row.representation_label_visible
            {
                violations.push(
                    StableSafePreviewTrustViolation::SurfaceMissingVisibleTruth {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.raw_rendered_distinction_meaningful
                && row.qualification == SurfaceQualification::Stable
                && row.source_representation_available
                && (!row.raw_view_path_visible
                    || !row
                        .transfer_actions
                        .contains(&RepresentationActionId::CopyRaw)
                    || (row.surface != SafePreviewConsumerSurface::SupportExport
                        && !row
                            .transfer_actions
                            .contains(&RepresentationActionId::CopyRendered)))
            {
                violations.push(
                    StableSafePreviewTrustViolation::RawRenderedChoiceCollapsed {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.surface.is_decision_surface()
                && row.qualification == SurfaceQualification::Stable
                && !row.auto_upgrade_blocked
            {
                violations.push(
                    StableSafePreviewTrustViolation::DecisionSurfaceCanAutoUpgrade {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.qualification == SurfaceQualification::Stable {
                for carrier in REQUIRED_TRUST_EVIDENCE_CARRIERS {
                    if !row.evidence_carriers.contains(&carrier) {
                        violations.push(StableSafePreviewTrustViolation::MissingEvidenceCarrier {
                            row_id: row.row_id.clone(),
                            carrier: carrier.as_str().to_string(),
                        });
                    }
                }
            }
            if let Some(contract) = contract_by_class.get(&row.trust_class) {
                for trigger in &contract.downgrade_triggers {
                    if row.qualification == SurfaceQualification::Stable
                        && !row.handled_downgrade_triggers.contains(trigger)
                    {
                        violations.push(StableSafePreviewTrustViolation::MissingDowngradeTrigger {
                            row_id: row.row_id.clone(),
                            trigger: trigger.as_str().to_string(),
                        });
                    }
                }
            }
            if matches!(
                row.trust_class,
                TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
            ) && row.qualification == SurfaceQualification::Stable
                && (!row.capability_summary_visible || !row.permission_summary_visible)
            {
                violations.push(
                    StableSafePreviewTrustViolation::ActiveSurfaceMissingCapabilityTruth {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.surface == SafePreviewConsumerSurface::SupportExport
                && row.qualification == SurfaceQualification::Stable
                && (matches!(
                    row.trust_class,
                    TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
                ) || row
                    .transfer_actions
                    .iter()
                    .any(|action| matches!(action, RepresentationActionId::CopyRendered)))
            {
                violations.push(StableSafePreviewTrustViolation::SupportExportCarriesActiveOrRenderedLiveContent {
                    row_id: row.row_id.clone(),
                });
            }
        }

        let case_kinds = self
            .transfer_cases
            .iter()
            .map(|case| case.case_kind)
            .collect::<BTreeSet<_>>();
        for kind in REQUIRED_TRANSFER_CASE_KINDS {
            if !case_kinds.contains(&kind) {
                violations.push(StableSafePreviewTrustViolation::MissingTransferCaseKind {
                    case_kind: kind.as_str().to_string(),
                });
            }
        }
        for case in &self.transfer_cases {
            if !case.trust_class_lineage_preserved || !case.origin_truth_preserved {
                violations.push(StableSafePreviewTrustViolation::TransferLosesLineage {
                    case_id: case.case_id.clone(),
                });
            }
            if matches!(
                case.source_trust_class,
                TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
            ) && !case.permission_truth_preserved
            {
                violations.push(
                    StableSafePreviewTrustViolation::TransferLosesPermissionTruth {
                        case_id: case.case_id.clone(),
                    },
                );
            }
            if matches!(
                case.case_kind,
                SafePreviewTransferCaseKind::Sanitized
                    | SafePreviewTransferCaseKind::TrustedLocal
                    | SafePreviewTransferCaseKind::IsolatedRemote
            ) && !case.preserves_raw_rendered_choice
            {
                violations.push(
                    StableSafePreviewTrustViolation::TransferCollapsesRepresentation {
                        case_id: case.case_id.clone(),
                    },
                );
            }
            if matches!(
                case.case_kind,
                SafePreviewTransferCaseKind::Downgrade | SafePreviewTransferCaseKind::Blocked
            ) && (case.downgrade_trigger.is_none()
                || case.effective_downgrade_state == SafePreviewDowngradeState::None)
            {
                violations.push(
                    StableSafePreviewTrustViolation::TransferMissingDowngradeTruth {
                        case_id: case.case_id.clone(),
                    },
                );
            }
            if matches!(
                case.source_trust_class,
                TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
            ) && case.support_export_safe
                && !matches!(
                    case.action_id,
                    RepresentationActionId::ExportSanitizedSnapshot
                        | RepresentationActionId::ExportMetadataOnly
                )
            {
                violations.push(
                    StableSafePreviewTrustViolation::SupportExportTransferNotSanitized {
                        case_id: case.case_id.clone(),
                    },
                );
            }
        }

        let status = if violations.is_empty() {
            StableSafePreviewTrustGateStatus::Pass
        } else {
            StableSafePreviewTrustGateStatus::Blocked
        };

        StableSafePreviewTrustValidationReport {
            record_kind: STABLE_SAFE_PREVIEW_TRUST_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: STABLE_SAFE_PREVIEW_TRUST_SCHEMA_VERSION,
            status,
            observed_trust_classes: contract_classes
                .iter()
                .map(|class| class.as_str().to_string())
                .collect(),
            observed_stable_surfaces: stable_surface_counts
                .keys()
                .map(|surface| surface.as_str().to_string())
                .collect(),
            observed_transfer_case_kinds: case_kinds
                .iter()
                .map(|kind| kind.as_str().to_string())
                .collect(),
            violations,
        }
    }
}

/// Stable gate status for safe-preview trust validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableSafePreviewTrustGateStatus {
    /// Packet satisfies the stable contract.
    Pass,
    /// Packet violates one or more stable rules.
    Blocked,
}

impl StableSafePreviewTrustGateStatus {
    /// Stable token used in reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Blocked => "blocked",
        }
    }
}

/// Validation report for stable safe-preview trust packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSafePreviewTrustValidationReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Gate status.
    pub status: StableSafePreviewTrustGateStatus,
    /// Trust classes observed in the contract.
    pub observed_trust_classes: Vec<String>,
    /// Stable surfaces observed in the matrix.
    pub observed_stable_surfaces: Vec<String>,
    /// Transfer case kinds observed in the fixture corpus.
    pub observed_transfer_case_kinds: Vec<String>,
    /// Validation violations.
    pub violations: Vec<StableSafePreviewTrustViolation>,
}

impl StableSafePreviewTrustValidationReport {
    /// Returns true when the stable gate passed.
    pub fn is_green(&self) -> bool {
        matches!(self.status, StableSafePreviewTrustGateStatus::Pass)
    }
}

/// Validation violation for stable safe-preview trust packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StableSafePreviewTrustViolation {
    /// Packet record kind is not the stable tag.
    InvalidRecordKind,
    /// Packet schema version is not supported.
    InvalidSchemaVersion,
    /// Packet does not carry the shared contract ref.
    MissingSharedContractRef,
    /// A required trust class is missing.
    MissingTrustClass {
        /// Missing trust-class token.
        trust_class: String,
    },
    /// A trust class is declared more than once.
    DuplicateTrustClass {
        /// Duplicate trust-class token.
        trust_class: String,
    },
    /// A required stable surface is missing.
    MissingStableSurface {
        /// Missing surface token.
        surface: String,
    },
    /// A stable surface appears more than once.
    DuplicateStableSurface {
        /// Duplicate surface token.
        surface: String,
    },
    /// Surface does not consume the shared contract ref.
    SurfaceDoesNotConsumeContract {
        /// Row id.
        row_id: String,
    },
    /// Surface hides trust class, representation, owner, or origin truth.
    SurfaceMissingVisibleTruth {
        /// Row id.
        row_id: String,
    },
    /// Raw and rendered choices collapsed where they differ materially.
    RawRenderedChoiceCollapsed {
        /// Row id.
        row_id: String,
    },
    /// Trust-sensitive decision surface can auto-upgrade.
    DecisionSurfaceCanAutoUpgrade {
        /// Row id.
        row_id: String,
    },
    /// Evidence carrier is missing.
    MissingEvidenceCarrier {
        /// Row id.
        row_id: String,
        /// Missing carrier token.
        carrier: String,
    },
    /// Surface does not handle a required downgrade trigger.
    MissingDowngradeTrigger {
        /// Row id.
        row_id: String,
        /// Missing trigger token.
        trigger: String,
    },
    /// Active surface hides capability or permission truth.
    ActiveSurfaceMissingCapabilityTruth {
        /// Row id.
        row_id: String,
    },
    /// Support/export attempts to carry live rendered active content.
    SupportExportCarriesActiveOrRenderedLiveContent {
        /// Row id.
        row_id: String,
    },
    /// A required transfer case kind is missing.
    MissingTransferCaseKind {
        /// Missing case-kind token.
        case_kind: String,
    },
    /// Transfer lost trust-class or origin lineage.
    TransferLosesLineage {
        /// Case id.
        case_id: String,
    },
    /// Active transfer lost permission or capability truth.
    TransferLosesPermissionTruth {
        /// Case id.
        case_id: String,
    },
    /// Transfer collapsed a meaningful raw/rendered choice.
    TransferCollapsesRepresentation {
        /// Case id.
        case_id: String,
    },
    /// Downgrade or blocked transfer lacks downgrade truth.
    TransferMissingDowngradeTruth {
        /// Case id.
        case_id: String,
    },
    /// Support/export transfer from active content is not sanitized or metadata-only.
    SupportExportTransferNotSanitized {
        /// Case id.
        case_id: String,
    },
}

/// Returns the canonical stable safe-preview trust packet.
pub fn stable_safe_preview_trust_packet() -> StableSafePreviewTrustPacket {
    StableSafePreviewTrustPacket {
        record_kind: STABLE_SAFE_PREVIEW_TRUST_PACKET_RECORD_KIND.to_string(),
        schema_version: STABLE_SAFE_PREVIEW_TRUST_SCHEMA_VERSION,
        shared_contract_ref: STABLE_SAFE_PREVIEW_SHARED_CONTRACT_REF.to_string(),
        schema_ref: STABLE_SAFE_PREVIEW_TRUST_SCHEMA_REF.to_string(),
        doc_ref: STABLE_SAFE_PREVIEW_TRUST_DOC_REF.to_string(),
        trust_class_contracts: trust_class_contracts(),
        surface_matrix: stable_surface_matrix(),
        transfer_cases: stable_transfer_cases(),
    }
}

fn required_trust_classes() -> [TrustClass; 4] {
    [
        TrustClass::RawText,
        TrustClass::SanitizedRich,
        TrustClass::TrustedLocalActive,
        TrustClass::IsolatedRemoteActive,
    ]
}

fn duplicate_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut counts = BTreeMap::<&str, usize>::new();
    for token in tokens {
        *counts.entry(token).or_default() += 1;
    }
    counts
        .into_iter()
        .filter_map(|(token, count)| (count > 1).then(|| token.to_string()))
        .collect()
}

fn base_cues() -> Vec<VisibleTrustCue> {
    vec![
        VisibleTrustCue::TrustClassBadge,
        VisibleTrustCue::RepresentationLabel,
        VisibleTrustCue::RawViewPath,
        VisibleTrustCue::OwnerIdentity,
        VisibleTrustCue::OriginBoundary,
        VisibleTrustCue::DowngradeExplanation,
    ]
}

fn active_cues() -> Vec<VisibleTrustCue> {
    let mut cues = base_cues();
    cues.extend([
        VisibleTrustCue::CapabilitySummary,
        VisibleTrustCue::PermissionSummary,
    ]);
    cues
}

fn active_downgrade_triggers() -> Vec<SafePreviewDowngradeTrigger> {
    vec![
        SafePreviewDowngradeTrigger::TrustLoss,
        SafePreviewDowngradeTrigger::PolicyDeny,
        SafePreviewDowngradeTrigger::Disconnect,
        SafePreviewDowngradeTrigger::OriginLoss,
        SafePreviewDowngradeTrigger::UnsupportedHost,
        SafePreviewDowngradeTrigger::BlockedActiveCapability,
        SafePreviewDowngradeTrigger::SupportExportBoundary,
    ]
}

fn trust_class_contracts() -> Vec<SafePreviewTrustClassContract> {
    vec![
        SafePreviewTrustClassContract {
            trust_class: TrustClass::RawText,
            allowed_behaviors: vec![
                SafePreviewAllowedBehavior::RenderExactBytes,
                SafePreviewAllowedBehavior::ShowInlineWarningOverlays,
                SafePreviewAllowedBehavior::RevealRawSource,
                SafePreviewAllowedBehavior::RevealEscapedSource,
                SafePreviewAllowedBehavior::BlockActiveContent,
            ],
            visible_cues: base_cues(),
            default_transfer_actions: vec![
                RepresentationActionId::CopyRaw,
                RepresentationActionId::CopyEscaped,
                RepresentationActionId::ExportMetadataOnly,
            ],
            upgrade_requirements: vec!["explicit_user_action".to_string()],
            downgrade_triggers: vec![
                SafePreviewDowngradeTrigger::PolicyDeny,
                SafePreviewDowngradeTrigger::OriginLoss,
                SafePreviewDowngradeTrigger::SupportExportBoundary,
            ],
            fallback_states: vec![
                SafePreviewDowngradeState::StaticSnapshot,
                SafePreviewDowngradeState::MetadataOnly,
            ],
        },
        SafePreviewTrustClassContract {
            trust_class: TrustClass::SanitizedRich,
            allowed_behaviors: vec![
                SafePreviewAllowedBehavior::RenderSanitizedMarkup,
                SafePreviewAllowedBehavior::ShowInlineWarningOverlays,
                SafePreviewAllowedBehavior::RevealRawSource,
                SafePreviewAllowedBehavior::RevealEscapedSource,
                SafePreviewAllowedBehavior::OpenStaticSnapshot,
                SafePreviewAllowedBehavior::BlockActiveContent,
            ],
            visible_cues: base_cues(),
            default_transfer_actions: vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::CopyRaw,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
            upgrade_requirements: vec![
                "explicit_user_elevation".to_string(),
                "policy_allows_active_content".to_string(),
            ],
            downgrade_triggers: vec![
                SafePreviewDowngradeTrigger::PolicyDeny,
                SafePreviewDowngradeTrigger::OriginLoss,
                SafePreviewDowngradeTrigger::BlockedActiveCapability,
                SafePreviewDowngradeTrigger::SupportExportBoundary,
            ],
            fallback_states: vec![
                SafePreviewDowngradeState::StaticSnapshot,
                SafePreviewDowngradeState::MetadataOnly,
            ],
        },
        SafePreviewTrustClassContract {
            trust_class: TrustClass::TrustedLocalActive,
            allowed_behaviors: vec![
                SafePreviewAllowedBehavior::RunLocalActiveContent,
                SafePreviewAllowedBehavior::RenderSanitizedMarkup,
                SafePreviewAllowedBehavior::RevealRawSource,
                SafePreviewAllowedBehavior::OpenStaticSnapshot,
                SafePreviewAllowedBehavior::BlockActiveContent,
            ],
            visible_cues: active_cues(),
            default_transfer_actions: vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::CopyRaw,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
            upgrade_requirements: vec![
                "trusted_workspace".to_string(),
                "explicit_user_elevation".to_string(),
                "policy_allows_active_content".to_string(),
                "declared_capability_sandbox".to_string(),
            ],
            downgrade_triggers: active_downgrade_triggers(),
            fallback_states: vec![
                SafePreviewDowngradeState::Sanitized,
                SafePreviewDowngradeState::StaticSnapshot,
                SafePreviewDowngradeState::MetadataOnly,
            ],
        },
        SafePreviewTrustClassContract {
            trust_class: TrustClass::IsolatedRemoteActive,
            allowed_behaviors: vec![
                SafePreviewAllowedBehavior::RunIsolatedRemoteActiveContent,
                SafePreviewAllowedBehavior::RenderSanitizedMarkup,
                SafePreviewAllowedBehavior::OpenStaticSnapshot,
                SafePreviewAllowedBehavior::BlockActiveContent,
            ],
            visible_cues: active_cues(),
            default_transfer_actions: vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
            upgrade_requirements: vec![
                "explicit_user_elevation".to_string(),
                "verified_origin_identity".to_string(),
                "declared_remote_origin_contract".to_string(),
                "live_connectivity".to_string(),
                "policy_approval".to_string(),
            ],
            downgrade_triggers: active_downgrade_triggers(),
            fallback_states: vec![
                SafePreviewDowngradeState::StaticSnapshot,
                SafePreviewDowngradeState::MetadataOnly,
                SafePreviewDowngradeState::Blocked,
            ],
        },
    ]
}

fn all_carriers() -> Vec<TrustEvidenceCarrier> {
    REQUIRED_TRUST_EVIDENCE_CARRIERS.to_vec()
}

fn stable_row(
    row_id: &str,
    surface: SafePreviewConsumerSurface,
    trust_class: TrustClass,
    visible_representation: RepresentationClass,
    raw_rendered_distinction_meaningful: bool,
    source_representation_available: bool,
    origin_boundary: OriginBoundaryClass,
    transfer_actions: Vec<RepresentationActionId>,
) -> SafePreviewSurfaceMatrixRow {
    let active = matches!(
        trust_class,
        TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
    );
    let handled_downgrade_triggers = match trust_class {
        TrustClass::RawText => vec![
            SafePreviewDowngradeTrigger::PolicyDeny,
            SafePreviewDowngradeTrigger::OriginLoss,
            SafePreviewDowngradeTrigger::SupportExportBoundary,
        ],
        TrustClass::SanitizedRich => vec![
            SafePreviewDowngradeTrigger::PolicyDeny,
            SafePreviewDowngradeTrigger::OriginLoss,
            SafePreviewDowngradeTrigger::BlockedActiveCapability,
            SafePreviewDowngradeTrigger::SupportExportBoundary,
        ],
        TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive => {
            active_downgrade_triggers()
        }
    };
    SafePreviewSurfaceMatrixRow {
        row_id: row_id.to_string(),
        surface,
        qualification: SurfaceQualification::Stable,
        consumed_contract_ref: STABLE_SAFE_PREVIEW_SHARED_CONTRACT_REF.to_string(),
        trust_class,
        visible_representation,
        raw_rendered_distinction_meaningful,
        source_representation_available,
        raw_view_path_visible: true,
        origin_boundary,
        owner_identity_visible: true,
        origin_identity_visible: true,
        trust_class_visible: true,
        representation_label_visible: true,
        capability_summary_visible: active,
        permission_summary_visible: active,
        transfer_actions,
        handled_downgrade_triggers,
        effective_downgrade_state: SafePreviewDowngradeState::None,
        auto_upgrade_blocked: true,
        evidence_carriers: all_carriers(),
    }
}

fn stable_surface_matrix() -> Vec<SafePreviewSurfaceMatrixRow> {
    vec![
        stable_row(
            "safe-preview:surface:editor",
            SafePreviewConsumerSurface::Editor,
            TrustClass::RawText,
            RepresentationClass::Raw,
            false,
            true,
            OriginBoundaryClass::LocalWorkspace,
            vec![
                RepresentationActionId::CopyRaw,
                RepresentationActionId::CopyEscaped,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:docs-help-preview",
            SafePreviewConsumerSurface::DocsHelpPreview,
            TrustClass::SanitizedRich,
            RepresentationClass::Rendered,
            true,
            true,
            OriginBoundaryClass::LocalDocsPack,
            vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::CopyRaw,
                RepresentationActionId::CopyEscaped,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:notebook-rich-output",
            SafePreviewConsumerSurface::NotebookRichOutput,
            TrustClass::TrustedLocalActive,
            RepresentationClass::Rendered,
            true,
            true,
            OriginBoundaryClass::TrustedLocalRuntime,
            vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::CopyRaw,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:preview-runtime",
            SafePreviewConsumerSurface::PreviewRuntime,
            TrustClass::IsolatedRemoteActive,
            RepresentationClass::Rendered,
            true,
            true,
            OriginBoundaryClass::RemoteProviderOrigin,
            vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::CopyRaw,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:marketplace-account-webview",
            SafePreviewConsumerSurface::MarketplaceAccountWebview,
            TrustClass::IsolatedRemoteActive,
            RepresentationClass::Rendered,
            true,
            false,
            OriginBoundaryClass::EmbeddedMarketplaceOrigin,
            vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:browser-runtime-viewer",
            SafePreviewConsumerSurface::BrowserRuntimeViewer,
            TrustClass::IsolatedRemoteActive,
            RepresentationClass::Rendered,
            true,
            false,
            OriginBoundaryClass::BrowserRuntimeOrigin,
            vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:support-export",
            SafePreviewConsumerSurface::SupportExport,
            TrustClass::SanitizedRich,
            RepresentationClass::Sanitized,
            true,
            true,
            OriginBoundaryClass::SupportExportSnapshot,
            vec![
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
                RepresentationActionId::CopyRaw,
            ],
        ),
        stable_row(
            "safe-preview:surface:install-review",
            SafePreviewConsumerSurface::InstallReview,
            TrustClass::RawText,
            RepresentationClass::Raw,
            false,
            true,
            OriginBoundaryClass::RemoteProviderOrigin,
            vec![
                RepresentationActionId::CopyRaw,
                RepresentationActionId::CopyEscaped,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:attach-review",
            SafePreviewConsumerSurface::AttachReview,
            TrustClass::RawText,
            RepresentationClass::Raw,
            false,
            true,
            OriginBoundaryClass::RemoteProviderOrigin,
            vec![
                RepresentationActionId::CopyRaw,
                RepresentationActionId::CopyEscaped,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:approval-review",
            SafePreviewConsumerSurface::ApprovalReview,
            TrustClass::SanitizedRich,
            RepresentationClass::Rendered,
            true,
            true,
            OriginBoundaryClass::RemoteProviderOrigin,
            vec![
                RepresentationActionId::CopyRendered,
                RepresentationActionId::CopyRaw,
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:publish-review",
            SafePreviewConsumerSurface::PublishReview,
            TrustClass::RawText,
            RepresentationClass::Raw,
            false,
            true,
            OriginBoundaryClass::RemoteProviderOrigin,
            vec![
                RepresentationActionId::CopyRaw,
                RepresentationActionId::CopyEscaped,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        stable_row(
            "safe-preview:surface:delete-review",
            SafePreviewConsumerSurface::DeleteReview,
            TrustClass::RawText,
            RepresentationClass::Raw,
            false,
            true,
            OriginBoundaryClass::SupportExportSnapshot,
            vec![
                RepresentationActionId::CopyRaw,
                RepresentationActionId::CopyEscaped,
                RepresentationActionId::ExportMetadataOnly,
            ],
        ),
        SafePreviewSurfaceMatrixRow {
            row_id: "safe-preview:surface:legacy-embedded-widget-drill".to_string(),
            surface: SafePreviewConsumerSurface::MarketplaceAccountWebview,
            qualification: SurfaceQualification::BelowStable,
            consumed_contract_ref: STABLE_SAFE_PREVIEW_SHARED_CONTRACT_REF.to_string(),
            trust_class: TrustClass::IsolatedRemoteActive,
            visible_representation: RepresentationClass::BlockedMetadataOnly,
            raw_rendered_distinction_meaningful: true,
            source_representation_available: false,
            raw_view_path_visible: false,
            origin_boundary: OriginBoundaryClass::EmbeddedMarketplaceOrigin,
            owner_identity_visible: true,
            origin_identity_visible: false,
            trust_class_visible: true,
            representation_label_visible: true,
            capability_summary_visible: false,
            permission_summary_visible: false,
            transfer_actions: vec![RepresentationActionId::ExportMetadataOnly],
            handled_downgrade_triggers: vec![
                SafePreviewDowngradeTrigger::OriginLoss,
                SafePreviewDowngradeTrigger::UnsupportedHost,
            ],
            effective_downgrade_state: SafePreviewDowngradeState::Blocked,
            auto_upgrade_blocked: true,
            evidence_carriers: all_carriers(),
        },
    ]
}

fn transfer_case(
    case_id: &str,
    case_kind: SafePreviewTransferCaseKind,
    surface: SafePreviewConsumerSurface,
    source_trust_class: TrustClass,
    action_id: RepresentationActionId,
    representation_class: RepresentationClass,
    visible_label: &str,
    preserves_raw_rendered_choice: bool,
) -> SafePreviewTransferCase {
    let active = matches!(
        source_trust_class,
        TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
    );
    SafePreviewTransferCase {
        case_id: case_id.to_string(),
        case_kind,
        surface,
        source_trust_class,
        action_id,
        representation_class,
        visible_label: visible_label.to_string(),
        preserves_raw_rendered_choice,
        trust_class_lineage_preserved: true,
        origin_truth_preserved: true,
        permission_truth_preserved: active,
        downgrade_trigger: None,
        effective_downgrade_state: SafePreviewDowngradeState::None,
        support_export_safe: matches!(
            action_id,
            RepresentationActionId::ExportSanitizedSnapshot
                | RepresentationActionId::ExportMetadataOnly
        ),
    }
}

fn stable_transfer_cases() -> Vec<SafePreviewTransferCase> {
    let mut cases = vec![
        transfer_case(
            "safe-preview:transfer:raw-only-copy",
            SafePreviewTransferCaseKind::RawOnly,
            SafePreviewConsumerSurface::Editor,
            TrustClass::RawText,
            RepresentationActionId::CopyRaw,
            RepresentationClass::Raw,
            "Copy raw text",
            true,
        ),
        transfer_case(
            "safe-preview:transfer:sanitized-docs-copy-rendered",
            SafePreviewTransferCaseKind::Sanitized,
            SafePreviewConsumerSurface::DocsHelpPreview,
            TrustClass::SanitizedRich,
            RepresentationActionId::CopyRendered,
            RepresentationClass::Rendered,
            "Copy rendered preview",
            true,
        ),
        transfer_case(
            "safe-preview:transfer:trusted-local-export-sanitized",
            SafePreviewTransferCaseKind::TrustedLocal,
            SafePreviewConsumerSurface::NotebookRichOutput,
            TrustClass::TrustedLocalActive,
            RepresentationActionId::ExportSanitizedSnapshot,
            RepresentationClass::Sanitized,
            "Export sanitized output snapshot",
            true,
        ),
        transfer_case(
            "safe-preview:transfer:isolated-remote-export-sanitized",
            SafePreviewTransferCaseKind::IsolatedRemote,
            SafePreviewConsumerSurface::PreviewRuntime,
            TrustClass::IsolatedRemoteActive,
            RepresentationActionId::ExportSanitizedSnapshot,
            RepresentationClass::Sanitized,
            "Export sanitized preview snapshot",
            true,
        ),
    ];
    cases.push(SafePreviewTransferCase {
        case_id: "safe-preview:transfer:remote-disconnect-static-snapshot".to_string(),
        case_kind: SafePreviewTransferCaseKind::Downgrade,
        surface: SafePreviewConsumerSurface::BrowserRuntimeViewer,
        source_trust_class: TrustClass::IsolatedRemoteActive,
        action_id: RepresentationActionId::ExportSanitizedSnapshot,
        representation_class: RepresentationClass::Sanitized,
        visible_label: "Export sanitized static snapshot".to_string(),
        preserves_raw_rendered_choice: true,
        trust_class_lineage_preserved: true,
        origin_truth_preserved: true,
        permission_truth_preserved: true,
        downgrade_trigger: Some(SafePreviewDowngradeTrigger::Disconnect),
        effective_downgrade_state: SafePreviewDowngradeState::StaticSnapshot,
        support_export_safe: true,
    });
    cases.push(SafePreviewTransferCase {
        case_id: "safe-preview:transfer:unsupported-host-metadata-only".to_string(),
        case_kind: SafePreviewTransferCaseKind::Blocked,
        surface: SafePreviewConsumerSurface::MarketplaceAccountWebview,
        source_trust_class: TrustClass::IsolatedRemoteActive,
        action_id: RepresentationActionId::ExportMetadataOnly,
        representation_class: RepresentationClass::BlockedMetadataOnly,
        visible_label: "Export metadata only".to_string(),
        preserves_raw_rendered_choice: true,
        trust_class_lineage_preserved: true,
        origin_truth_preserved: true,
        permission_truth_preserved: true,
        downgrade_trigger: Some(SafePreviewDowngradeTrigger::UnsupportedHost),
        effective_downgrade_state: SafePreviewDowngradeState::Blocked,
        support_export_safe: true,
    });
    cases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_packet_validates() {
        let packet = stable_safe_preview_trust_packet();
        let report = packet.validate();
        assert!(report.is_green(), "{:#?}", report.violations);
    }

    #[test]
    fn support_export_cannot_claim_active_content() {
        let mut packet = stable_safe_preview_trust_packet();
        let row = packet
            .surface_matrix
            .iter_mut()
            .find(|row| row.surface == SafePreviewConsumerSurface::SupportExport)
            .expect("support row");
        row.trust_class = TrustClass::TrustedLocalActive;
        let report = packet.validate();
        assert!(report.violations.iter().any(|violation| matches!(
            violation,
            StableSafePreviewTrustViolation::SupportExportCarriesActiveOrRenderedLiveContent { .. }
        )));
    }

    #[test]
    fn rendered_rows_must_offer_raw_and_rendered_copy() {
        let mut packet = stable_safe_preview_trust_packet();
        let row = packet
            .surface_matrix
            .iter_mut()
            .find(|row| row.surface == SafePreviewConsumerSurface::DocsHelpPreview)
            .expect("docs row");
        row.transfer_actions
            .retain(|action| *action != RepresentationActionId::CopyRaw);
        let report = packet.validate();
        assert!(report.violations.iter().any(|violation| matches!(
            violation,
            StableSafePreviewTrustViolation::RawRenderedChoiceCollapsed { .. }
        )));
    }
}
