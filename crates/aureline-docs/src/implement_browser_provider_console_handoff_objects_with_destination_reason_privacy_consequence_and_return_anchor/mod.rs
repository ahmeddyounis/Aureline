//! Browser / provider-console handoff objects with destination reason, privacy
//! consequence, return anchor, and no-hidden-context-share guarantees.
//!
//! Every claimed M5 external documentation or provider-console exit — a
//! docs-browser open, a help/about portal link, an AI-answer citation jump, or a
//! provider-console pivot — must route through exactly one [`BrowserHandoff`]
//! object rather than a raw URL jump. A handoff object makes a boundary crossing
//! a typed, reviewable record: it names the governed surface the reader is
//! leaving (`source_surface`, `source_identity_ref`, `source_class`), the
//! destination class it opens ([`HandoffDestinationClass`]), *why* in-product
//! viewing was insufficient (`destination_reason` + `destination_reason_note`),
//! exactly what context does or does not cross the boundary
//! ([`SharedContext`] + `privacy_consequence`), the current trust and policy
//! posture (`trust_class` + [`HandoffPolicyPosture`]), and how the reader gets
//! back ([`ReturnAnchor`] with an optional follow-up note).
//!
//! The lane enforces the boundary-honesty invariants the matrix froze:
//!
//! * **No hidden context share.** A handoff never silently exfiltrates raw code
//!   selections, private README text, unpublished ADR content, or prompt context
//!   as part of ordinary docs navigation. Those four raw vectors must never cross
//!   the boundary, and an ordinary-navigation handoff must not share even the
//!   user's query terms; query-term sharing is allowed only when the handoff is
//!   explicitly user-initiated and the sharing is disclosed.
//! * **Every exit is reviewed.** A raw browser open, provider-console pivot, or
//!   docs fallback may not bypass explicit handoff review, and a policy-blocked or
//!   unavailable destination may not be presented as an available action.
//! * **Identity survives the round trip.** The handoff keeps a stable,
//!   export-safe identity so help, support-export, and reopened docs-history
//!   surfaces can reconstruct the prior handoff reason and return anchor from one
//!   object rather than flattening it into ordinary navigation.
//!
//! [`BrowserHandoffPacket::materialize`] computes the validation findings and the
//! promotion state (`stable`, `narrowed_below_stable`, or `blocks_stable`) from
//! the input, so a handoff that leaks context, bypasses review, drops its return
//! anchor, presents a blocked destination as available, or a support/history
//! projection that drops a handoff automatically narrows or blocks before it
//! reaches a consumer surface. The packet is an inspectable, serde-serializable
//! truth packet: it carries no raw URLs, raw callback bodies, raw page bodies,
//! raw code selections, prompt text, raw provider payloads, or credentials — only
//! metadata, opaque refs, reason/privacy/posture vocabulary, return anchors, and
//! contract refs.
//!
//! The controlled vocabularies reuse the canonical source-class, trust-class,
//! browser-handoff-reason, and privacy-consequence tokens frozen by the
//! docs-contracts matrix, and the destination-class tokens stay aligned with the
//! integration-level browser-handoff packet contract, so docs browser, AI,
//! help/about, support, history, and extension surfaces project one handoff
//! object instead of minting parallel tokens.
//!
//! The boundary schema is
//! [`schemas/docs/implement-browser-provider-console-handoff-objects-with-destination-reason-privacy-consequence-and-return-anchor.schema.json`](../../../../schemas/docs/implement-browser-provider-console-handoff-objects-with-destination-reason-privacy-consequence-and-return-anchor.schema.json).
//! The contract doc is
//! [`docs/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor.md`](../../../../docs/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/`](../../../../fixtures/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    DocsContractBrowserHandoffPrivacyConsequence, DocsContractBrowserHandoffReason,
    DocsContractSourceClass, DocsContractTrustClass,
};

/// Stable record-kind tag carried by [`BrowserHandoffPacket`].
pub const BROWSER_HANDOFF_OBJECTS_RECORD_KIND: &str =
    "browser_provider_console_handoff_objects_packet";

/// Stable record-kind tag carried by [`BrowserHandoffSupportExport`].
pub const BROWSER_HANDOFF_OBJECTS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "browser_provider_console_handoff_objects_support_export";

/// Schema version for browser/provider-console handoff-object records.
pub const BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BROWSER_HANDOFF_OBJECTS_SCHEMA_REF: &str =
    "schemas/docs/implement-browser-provider-console-handoff-objects-with-destination-reason-privacy-consequence-and-return-anchor.schema.json";

/// Repo-relative path of the contract doc.
pub const BROWSER_HANDOFF_OBJECTS_DOC_REF: &str =
    "docs/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor.md";

/// Repo-relative path of the checked support-export artifact.
pub const BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF: &str =
    "artifacts/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const BROWSER_HANDOFF_OBJECTS_SUMMARY_REF: &str =
    "artifacts/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor.md";

/// Repo-relative path of the protected fixture directory.
pub const BROWSER_HANDOFF_OBJECTS_FIXTURE_DIR: &str =
    "fixtures/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor";

/// Repo-relative path of the frozen docs-contracts matrix the lane consumes.
pub const BROWSER_HANDOFF_OBJECTS_MATRIX_CONTRACT_REF: &str =
    "schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json";

/// Repo-relative path of the integration browser-handoff packet contract whose
/// destination-class vocabulary this lane stays aligned with.
pub const BROWSER_HANDOFF_OBJECTS_INTEGRATION_CONTRACT_REF: &str =
    "schemas/integration/browser_handoff_packet.schema.json";

/// A governed in-product surface a browser handoff can originate from.
///
/// These name the docs/help/AI/provider-console exits the lane requires to route
/// through a handoff object; `review_surface` and `support_history` are
/// additional origins the same object model covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffSourceSurface {
    /// The docs browser / reader surface.
    DocsBrowser,
    /// The help / about surface.
    HelpAbout,
    /// An AI answer / citation surface.
    AiAnswer,
    /// A provider-console pivot (e.g. opening the AI or admin provider console).
    ProviderConsolePivot,
    /// A hosted review surface.
    ReviewSurface,
    /// A reopened docs-history / support surface replaying a prior handoff.
    SupportHistory,
}

impl HandoffSourceSurface {
    /// Every source surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DocsBrowser,
        Self::HelpAbout,
        Self::AiAnswer,
        Self::ProviderConsolePivot,
        Self::ReviewSurface,
        Self::SupportHistory,
    ];

    /// The exit surfaces that MUST each route at least one handoff: the
    /// docs/help/AI/provider-console exits the lane governs.
    pub const REQUIRED_EXITS: [Self; 4] = [
        Self::DocsBrowser,
        Self::HelpAbout,
        Self::AiAnswer,
        Self::ProviderConsolePivot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::HelpAbout => "help_about",
            Self::AiAnswer => "ai_answer",
            Self::ProviderConsolePivot => "provider_console_pivot",
            Self::ReviewSurface => "review_surface",
            Self::SupportHistory => "support_history",
        }
    }
}

/// The class of external surface a browser handoff opens.
///
/// Tokens stay aligned with the integration-level browser-handoff packet
/// contract's `destination_class` vocabulary so a docs/help/AI handoff and a
/// system-browser launch describe the same destination the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDestinationClass {
    /// A documentation site or portal (vendor / framework / product docs).
    DocsOrPortalWeb,
    /// A code-review host / hosted diff view.
    CodeHostWeb,
    /// An issue tracker.
    IssueTrackerWeb,
    /// A package registry.
    PackageRegistryWeb,
    /// An AI provider console / web surface.
    AiProviderWeb,
    /// A managed / admin provider console.
    ManagedAdminWeb,
    /// A generic external web destination (last resort).
    ExternalGenericWeb,
}

impl HandoffDestinationClass {
    /// Every destination class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DocsOrPortalWeb,
        Self::CodeHostWeb,
        Self::IssueTrackerWeb,
        Self::PackageRegistryWeb,
        Self::AiProviderWeb,
        Self::ManagedAdminWeb,
        Self::ExternalGenericWeb,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsOrPortalWeb => "docs_or_portal_web",
            Self::CodeHostWeb => "code_host_web",
            Self::IssueTrackerWeb => "issue_tracker_web",
            Self::PackageRegistryWeb => "package_registry_web",
            Self::AiProviderWeb => "ai_provider_web",
            Self::ManagedAdminWeb => "managed_admin_web",
            Self::ExternalGenericWeb => "external_generic_web",
        }
    }

    /// Whether this destination is a provider console.
    pub const fn is_provider_console(self) -> bool {
        matches!(self, Self::AiProviderWeb | Self::ManagedAdminWeb)
    }
}

/// The current trust / policy posture of a browser handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffPolicyPosture {
    /// Explicit handoff review passed and the handoff is an available action.
    AllowedExplicit,
    /// The handoff is allowed but requires an explicit user confirmation step.
    RequiresConfirmation,
    /// The handoff is disallowed by policy and disclosed as blocked.
    BlockedByPolicy,
    /// The destination is unreachable / unavailable and disclosed as such.
    UnavailableDisclosed,
}

impl HandoffPolicyPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AllowedExplicit,
        Self::RequiresConfirmation,
        Self::BlockedByPolicy,
        Self::UnavailableDisclosed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedExplicit => "allowed_explicit",
            Self::RequiresConfirmation => "requires_confirmation",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::UnavailableDisclosed => "unavailable_disclosed",
        }
    }

    /// Whether the handoff may be presented as an available action.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::AllowedExplicit | Self::RequiresConfirmation)
    }

    /// Whether the handoff is blocked by policy.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BlockedByPolicy)
    }

    /// Whether the destination is unavailable / unreachable.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::UnavailableDisclosed)
    }
}

/// Where the reader returns to after a browser handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnAnchorKind {
    /// Back to the docs browser shell.
    BackToDocsBrowser,
    /// Back to the help / about surface.
    BackToHelpAbout,
    /// Back to the AI answer the citation came from.
    BackToAiAnswer,
    /// Back to the review panel.
    BackToReviewPanel,
    /// Back to the workspace / editor.
    BackToWorkspace,
    /// Back to the provider-console panel inside the product.
    BackToProviderPanel,
}

impl ReturnAnchorKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackToDocsBrowser => "back_to_docs_browser",
            Self::BackToHelpAbout => "back_to_help_about",
            Self::BackToAiAnswer => "back_to_ai_answer",
            Self::BackToReviewPanel => "back_to_review_panel",
            Self::BackToWorkspace => "back_to_workspace",
            Self::BackToProviderPanel => "back_to_provider_panel",
        }
    }
}

/// How the reader returns from a browser handoff (the return-path guarantee).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnAnchor {
    /// Kind of return anchor.
    pub anchor_kind: ReturnAnchorKind,
    /// Stable destination ref the reader returns to (no raw URL).
    pub anchor_ref: String,
    /// Human-readable return label.
    pub label: String,
    /// Optional follow-up note describing what to do after returning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_up_note: Option<String>,
}

impl ReturnAnchor {
    /// True when the return anchor names a non-empty ref and label.
    pub fn is_well_formed(&self) -> bool {
        !self.anchor_ref.trim().is_empty() && !self.label.trim().is_empty()
    }
}

/// Exactly what context does or does not cross the boundary on a handoff.
///
/// The four `shares_raw_*` flags are the named exfiltration vectors that must
/// never cross during a docs/help/AI handoff; the validator blocks any handoff
/// that sets one. `shares_resolved_destination_ref` and `shares_user_query_terms`
/// are the only context a handoff may carry, and query-term sharing is allowed
/// only on an explicitly user-initiated, disclosed handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedContext {
    /// Only the opaque resolved destination ref / anchor crosses the boundary.
    pub shares_resolved_destination_ref: bool,
    /// The user's query terms cross the boundary (must be user-initiated and
    /// disclosed).
    pub shares_user_query_terms: bool,
    /// A raw code selection crosses the boundary. Must always be false.
    pub shares_raw_code_selection: bool,
    /// Private README text crosses the boundary. Must always be false.
    pub shares_private_readme_text: bool,
    /// Unpublished ADR content crosses the boundary. Must always be false.
    pub shares_unpublished_adr_content: bool,
    /// Prompt context crosses the boundary. Must always be false.
    pub shares_prompt_context: bool,
}

impl SharedContext {
    /// A shared-context object that crosses nothing.
    pub const NOTHING: Self = Self {
        shares_resolved_destination_ref: false,
        shares_user_query_terms: false,
        shares_raw_code_selection: false,
        shares_private_readme_text: false,
        shares_unpublished_adr_content: false,
        shares_prompt_context: false,
    };

    /// True when one of the four raw workspace-context vectors crosses.
    pub const fn leaks_raw_workspace_context(self) -> bool {
        self.shares_raw_code_selection
            || self.shares_private_readme_text
            || self.shares_unpublished_adr_content
            || self.shares_prompt_context
    }

    /// True when nothing at all crosses the boundary.
    pub const fn crosses_nothing(self) -> bool {
        !self.shares_resolved_destination_ref
            && !self.shares_user_query_terms
            && !self.leaks_raw_workspace_context()
    }
}

/// One browser / provider-console handoff object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoff {
    /// Stable, export-safe handoff id.
    pub handoff_id: String,
    /// The governed surface the reader is leaving.
    pub source_surface: HandoffSourceSurface,
    /// Stable identity ref of the originating surface / node (no raw body).
    pub source_identity_ref: String,
    /// Source class of the material the reader was viewing.
    pub source_class: DocsContractSourceClass,
    /// The destination class the handoff opens.
    pub destination_class: HandoffDestinationClass,
    /// Opaque destination ref (no raw URL).
    pub destination_ref: String,
    /// Why the product is leaving a governed surface.
    pub destination_reason: DocsContractBrowserHandoffReason,
    /// Human-readable note: why in-product viewing was insufficient.
    pub destination_reason_note: String,
    /// The privacy consequence of the handoff.
    pub privacy_consequence: DocsContractBrowserHandoffPrivacyConsequence,
    /// Exactly what context does or does not cross the boundary.
    pub shared_context: SharedContext,
    /// Trust class of the destination.
    pub trust_class: DocsContractTrustClass,
    /// Current policy posture.
    pub policy_posture: HandoffPolicyPosture,
    /// Human-readable trust / policy disclosure note.
    pub policy_disclosure_note: String,
    /// How the reader returns.
    pub return_anchor: ReturnAnchor,
    /// True when the exit happens as part of ordinary docs navigation (rather
    /// than an explicitly user-initiated action).
    pub ordinary_navigation: bool,
    /// True when the reader explicitly initiated this handoff.
    pub user_initiated: bool,
    /// True when the handoff is offered as an actionable open.
    pub offered_as_actionable: bool,
    /// True when the handoff went through explicit handoff review.
    pub routed_through_handoff_review: bool,
    /// True when raw boundary material is excluded from this object.
    pub raw_boundary_material_excluded: bool,
}

impl BrowserHandoff {
    /// True when every required identity field is present.
    pub fn is_well_formed(&self) -> bool {
        !self.handoff_id.trim().is_empty()
            && !self.source_identity_ref.trim().is_empty()
            && !self.destination_ref.trim().is_empty()
    }

    /// True when the declared privacy consequence agrees with the shared-context
    /// object.
    pub fn privacy_consequence_consistent(&self) -> bool {
        use DocsContractBrowserHandoffPrivacyConsequence as P;
        let sc = self.shared_context;
        match self.privacy_consequence {
            P::NoContextShared => sc.crosses_nothing(),
            P::ScopedUrlOnly => {
                sc.shares_resolved_destination_ref
                    && !sc.shares_user_query_terms
                    && !sc.leaks_raw_workspace_context()
            }
            P::QueryTermsDisclosed => {
                sc.shares_user_query_terms
                    && self.user_initiated
                    && !sc.leaks_raw_workspace_context()
            }
            P::IsolatedSession => !sc.shares_user_query_terms && !sc.leaks_raw_workspace_context(),
            // A blocked share crosses nothing — the attempt was prevented.
            P::SharedContextBlocked => sc.crosses_nothing(),
        }
    }

    /// True when a raw workspace-context vector crosses the boundary.
    pub fn leaks_hidden_context(&self) -> bool {
        self.shared_context.leaks_raw_workspace_context()
    }

    /// True when ordinary navigation shares more than the resolved destination
    /// ref (query terms or raw context).
    pub fn ordinary_navigation_overshares(&self) -> bool {
        self.ordinary_navigation
            && (self.shared_context.shares_user_query_terms
                || self.shared_context.leaks_raw_workspace_context())
    }

    /// True when the handoff bypassed explicit handoff review.
    pub fn bypasses_review(&self) -> bool {
        !self.routed_through_handoff_review
    }

    /// True when a blocked / unavailable destination is presented as actionable.
    pub fn presents_unavailable_as_available(&self) -> bool {
        (self.policy_posture.is_blocked() || self.policy_posture.is_unavailable())
            && self.offered_as_actionable
    }

    /// True when the handoff is honestly disclosed as blocked / unavailable and
    /// therefore narrows below stable.
    fn is_unavailable_narrowed(&self) -> bool {
        (self.policy_posture.is_blocked() || self.policy_posture.is_unavailable())
            && !self.offered_as_actionable
    }

    /// True when a context-share was honestly blocked, narrowing below stable.
    fn is_shared_context_blocked_narrowed(&self) -> bool {
        self.privacy_consequence
            == DocsContractBrowserHandoffPrivacyConsequence::SharedContextBlocked
    }
}

/// A consumer surface that reuses or reconstructs the handoff object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserHandoffConsumerSurface {
    /// The browser companion / handoff follow-up surface.
    BrowserCompanion,
    /// The help / about surface.
    HelpAbout,
    /// The support / export packet.
    SupportExport,
    /// The reopened docs-history surface.
    DocsHistory,
    /// Diagnostics or telemetry.
    Diagnostics,
    /// Extension API consumer.
    ExtensionApi,
}

impl BrowserHandoffConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BrowserCompanion,
        Self::HelpAbout,
        Self::SupportExport,
        Self::DocsHistory,
        Self::Diagnostics,
        Self::ExtensionApi,
    ];

    /// Surfaces that MUST be able to reconstruct prior handoff reason and return
    /// anchor from one stable handoff object.
    pub const REQUIRED_RECONSTRUCTION: [Self; 3] =
        [Self::HelpAbout, Self::SupportExport, Self::DocsHistory];

    /// Surfaces whose projection must reference every handoff so an export or a
    /// reopened history never silently drops a handoff.
    pub const FULL_COVERAGE: [Self; 2] = [Self::SupportExport, Self::DocsHistory];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserCompanion => "browser_companion",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
            Self::DocsHistory => "docs_history",
            Self::Diagnostics => "diagnostics",
            Self::ExtensionApi => "extension_api",
        }
    }

    /// Whether this surface must reference every handoff in its projection.
    pub fn requires_full_coverage(self) -> bool {
        Self::FULL_COVERAGE.contains(&self)
    }
}

/// One per-surface projection asserting a surface reuses the shared handoff
/// objects and can reconstruct the handoff reason and return anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffConsumerProjection {
    /// Consumer surface.
    pub surface: BrowserHandoffConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id this projection belongs to.
    pub packet_id_ref: String,
    /// True when the surface reuses the shared handoff objects.
    pub reuses_shared_handoff_object: bool,
    /// True when the surface preserves the destination reason.
    pub preserves_destination_reason: bool,
    /// True when the surface preserves the return anchor.
    pub preserves_return_anchor: bool,
    /// True when the surface preserves the privacy consequence.
    pub preserves_privacy_consequence: bool,
    /// Handoff ids this surface projects.
    pub handoff_id_refs: Vec<String>,
}

impl BrowserHandoffConsumerProjection {
    /// True when the projection preserves every required flag.
    pub fn preserves_required_flags(&self) -> bool {
        self.reuses_shared_handoff_object
            && self.preserves_destination_reason
            && self.preserves_return_anchor
            && self.preserves_privacy_consequence
    }
}

/// Derived promotion state of a handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserHandoffPromotionState {
    /// All invariants hold; the packet certifies a clean stable claim.
    Stable,
    /// A non-fatal narrowing applies (a blocked / unavailable destination or a
    /// blocked context share); the claim is narrowed below stable.
    NarrowedBelowStable,
    /// A blocking invariant failed; the packet may not claim stable.
    BlocksStable,
}

impl BrowserHandoffPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserHandoffValidationSeverity {
    /// Blocks the stable claim.
    Blocker,
    /// Narrows the claim below stable.
    Warning,
}

impl BrowserHandoffValidationSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Warning => "warning",
        }
    }
}

/// Closed set of validation finding kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserHandoffValidationKind {
    /// Record kind does not match the contract.
    WrongRecordKind,
    /// Schema version does not match the contract.
    WrongSchemaVersion,
    /// Packet identity is incomplete.
    MissingPacketIdentity,
    /// Source contract refs omit the schema or contract doc.
    MissingSourceContracts,
    /// Packet declares no handoff objects.
    MissingHandoffObjects,
    /// A handoff object drops a required identity field.
    HandoffObjectIncomplete,
    /// Two handoff objects share an id.
    DuplicateHandoffId,
    /// A handoff drops the note explaining why in-product viewing was insufficient.
    DestinationReasonMissing,
    /// A handoff drops its return anchor (return-path safety violation).
    ReturnAnchorMissing,
    /// A handoff drops its trust / policy disclosure note.
    TrustPolicyPostureMissing,
    /// The declared privacy consequence disagrees with the shared-context object.
    PrivacyConsequenceInconsistent,
    /// A raw code selection, private README, unpublished ADR, or prompt context
    /// crosses the boundary.
    HiddenContextShareDetected,
    /// Ordinary navigation shares query terms or workspace context.
    OrdinaryNavigationSharesContext,
    /// A blocked or unavailable destination is presented as available.
    BlockedHandoffPresentedAvailable,
    /// A raw browser open / provider-console pivot / docs fallback bypassed review.
    RawBrowserOpenBypass,
    /// A required docs/help/AI/provider-console exit has no handoff.
    ExitCoverageMissing,
    /// A required reconstruction surface has no projection.
    RequiredReconstructionSurfaceMissing,
    /// A reconstruction projection drops a required preservation flag.
    ReconstructionProjectionDropsHandoff,
    /// A projection references a different packet.
    ConsumerProjectionPacketIdMismatch,
    /// A projection references an unknown handoff id.
    ConsumerProjectionOrphanHandoffRef,
    /// A full-coverage surface (support export / history) drops a handoff.
    HistoryReconstructionDropsHandoff,
    /// Raw boundary material is present in the export.
    RawBoundaryMaterialPresent,
    /// A handoff is honestly disclosed as blocked / unavailable and narrows.
    HandoffUnavailableNarrowed,
    /// A context share was honestly blocked and narrows.
    SharedContextBlockedNarrowed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl BrowserHandoffValidationKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingHandoffObjects => "missing_handoff_objects",
            Self::HandoffObjectIncomplete => "handoff_object_incomplete",
            Self::DuplicateHandoffId => "duplicate_handoff_id",
            Self::DestinationReasonMissing => "destination_reason_missing",
            Self::ReturnAnchorMissing => "return_anchor_missing",
            Self::TrustPolicyPostureMissing => "trust_policy_posture_missing",
            Self::PrivacyConsequenceInconsistent => "privacy_consequence_inconsistent",
            Self::HiddenContextShareDetected => "hidden_context_share_detected",
            Self::OrdinaryNavigationSharesContext => "ordinary_navigation_shares_context",
            Self::BlockedHandoffPresentedAvailable => "blocked_handoff_presented_available",
            Self::RawBrowserOpenBypass => "raw_browser_open_bypass",
            Self::ExitCoverageMissing => "exit_coverage_missing",
            Self::RequiredReconstructionSurfaceMissing => "required_reconstruction_surface_missing",
            Self::ReconstructionProjectionDropsHandoff => "reconstruction_projection_drops_handoff",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::ConsumerProjectionOrphanHandoffRef => "consumer_projection_orphan_handoff_ref",
            Self::HistoryReconstructionDropsHandoff => "history_reconstruction_drops_handoff",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::HandoffUnavailableNarrowed => "handoff_unavailable_narrowed",
            Self::SharedContextBlockedNarrowed => "shared_context_blocked_narrowed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the handoff validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffValidationFinding {
    /// Closed finding kind.
    pub finding_kind: BrowserHandoffValidationKind,
    /// Finding severity.
    pub severity: BrowserHandoffValidationSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl BrowserHandoffValidationFinding {
    fn blocker(finding_kind: BrowserHandoffValidationKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BrowserHandoffValidationSeverity::Blocker,
            summary: summary.into(),
        }
    }

    fn warning(finding_kind: BrowserHandoffValidationKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BrowserHandoffValidationSeverity::Warning,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`BrowserHandoffPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Browser / provider-console handoff objects.
    pub handoffs: Vec<BrowserHandoff>,
    /// Per-surface projections.
    pub consumer_projections: Vec<BrowserHandoffConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

/// Export-safe browser / provider-console handoff-object packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffPacket {
    /// Record kind; must equal [`BROWSER_HANDOFF_OBJECTS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Browser / provider-console handoff objects.
    pub handoffs: Vec<BrowserHandoff>,
    /// Per-surface projections.
    pub consumer_projections: Vec<BrowserHandoffConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Derived promotion state.
    pub promotion_state: BrowserHandoffPromotionState,
    /// Validation findings.
    #[serde(default)]
    pub validation_findings: Vec<BrowserHandoffValidationFinding>,
}

impl BrowserHandoffPacket {
    /// Materializes the packet and records its derived findings and promotion
    /// state.
    pub fn materialize(input: BrowserHandoffPacketInput) -> Self {
        let mut packet = Self {
            record_kind: BROWSER_HANDOFF_OBJECTS_RECORD_KIND.to_owned(),
            schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            generated_at: input.generated_at,
            handoffs: input.handoffs,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            promotion_state: BrowserHandoffPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet's invariants, including the stored promotion state.
    pub fn validate(&self) -> Vec<BrowserHandoffValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker validation findings exist.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BrowserHandoffValidationSeverity::Blocker)
    }

    /// Returns true when the packet certifies the clean stable claim.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == BrowserHandoffPromotionState::Stable && self.validate().is_empty()
    }

    /// Returns the exit surfaces covered by at least one handoff.
    pub fn covered_exits(&self) -> Vec<HandoffSourceSurface> {
        let mut set = BTreeSet::new();
        for handoff in &self.handoffs {
            set.insert(handoff.source_surface);
        }
        set.into_iter().collect()
    }

    /// Returns true when at least one projection reconstructs this packet for
    /// `surface`.
    pub fn has_projection_for(&self, surface: BrowserHandoffConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.surface == surface
                && projection.packet_id_ref == self.packet_id
                && projection.preserves_required_flags()
        })
    }

    /// Returns the handoff with the given id, if present.
    pub fn handoff(&self, handoff_id: &str) -> Option<&BrowserHandoff> {
        self.handoffs
            .iter()
            .find(|handoff| handoff.handoff_id == handoff_id)
    }

    /// Wraps the packet in an export-safe support export.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> BrowserHandoffSupportExport {
        BrowserHandoffSupportExport {
            record_kind: BROWSER_HANDOFF_OBJECTS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            handoff_identity_preserved: true,
            return_anchor_preserved: true,
            export_packet: self.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("browser handoff packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Browser / Provider-Console Handoff Objects\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} validation findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Handoffs: {} / Projections: {}\n",
            self.handoffs.len(),
            self.consumer_projections.len()
        ));
        out.push_str("\n## Handoffs\n\n");
        for handoff in &self.handoffs {
            out.push_str(&format!(
                "- **{}** (`{}`): from `{}` to `{}`\n",
                handoff.handoff_id,
                handoff.source_identity_ref,
                handoff.source_surface.as_str(),
                handoff.destination_class.as_str(),
            ));
            out.push_str(&format!(
                "   - reason `{}`: {}\n",
                handoff.destination_reason.as_str(),
                handoff.destination_reason_note,
            ));
            out.push_str(&format!(
                "   - privacy `{}` / trust `{}` / policy `{}`\n",
                handoff.privacy_consequence.as_str(),
                handoff.trust_class.as_str(),
                handoff.policy_posture.as_str(),
            ));
            out.push_str(&format!(
                "   - return `{}`: {}\n",
                handoff.return_anchor.anchor_kind.as_str(),
                handoff.return_anchor.label,
            ));
        }
        out
    }

    fn derived_findings(&self, check_promotion: bool) -> Vec<BrowserHandoffValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != BROWSER_HANDOFF_OBJECTS_RECORD_KIND {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::WrongRecordKind,
                "record kind does not match the browser-handoff contract",
            ));
        }
        if self.schema_version != BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::WrongSchemaVersion,
                "schema version does not match the browser-handoff contract",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::MissingPacketIdentity,
                "packet identity is incomplete",
            ));
        }

        self.validate_source_contracts(&mut findings);
        self.validate_handoffs(&mut findings);
        self.validate_exit_coverage(&mut findings);
        self.validate_projections(&mut findings);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("browser handoff packet serializes"),
        ) {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::RawBoundaryMaterialPresent,
                "export contains forbidden raw boundary material",
            ));
        }

        if check_promotion {
            let derived = promotion_state_for(&findings);
            if self.promotion_state != derived {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::PromotionStateMismatch,
                    "stored promotion state disagrees with derived findings",
                ));
            }
        }

        findings
    }

    fn validate_source_contracts(&self, findings: &mut Vec<BrowserHandoffValidationFinding>) {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(BROWSER_HANDOFF_OBJECTS_SCHEMA_REF)
            || !refs.contains(BROWSER_HANDOFF_OBJECTS_DOC_REF)
        {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::MissingSourceContracts,
                "source contract refs omit the schema or contract doc",
            ));
        }
    }

    fn validate_handoffs(&self, findings: &mut Vec<BrowserHandoffValidationFinding>) {
        if self.handoffs.is_empty() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::MissingHandoffObjects,
                "packet must declare at least one handoff object",
            ));
        }

        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        for handoff in &self.handoffs {
            if !handoff.is_well_formed() {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::HandoffObjectIncomplete,
                    format!(
                        "handoff {} drops a required identity field",
                        handoff.handoff_id
                    ),
                ));
            }
            if !handoff.handoff_id.trim().is_empty()
                && !seen_ids.insert(handoff.handoff_id.as_str())
            {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::DuplicateHandoffId,
                    format!("duplicate handoff id {}", handoff.handoff_id),
                ));
            }
            self.validate_one_handoff(handoff, findings);
        }
    }

    fn validate_one_handoff(
        &self,
        handoff: &BrowserHandoff,
        findings: &mut Vec<BrowserHandoffValidationFinding>,
    ) {
        if handoff.destination_reason_note.trim().is_empty() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::DestinationReasonMissing,
                format!(
                    "handoff {} must explain why in-product viewing was insufficient",
                    handoff.handoff_id
                ),
            ));
        }
        if !handoff.return_anchor.is_well_formed() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::ReturnAnchorMissing,
                format!(
                    "handoff {} must keep a return anchor (return-path safety)",
                    handoff.handoff_id
                ),
            ));
        }
        if handoff.policy_disclosure_note.trim().is_empty() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::TrustPolicyPostureMissing,
                format!(
                    "handoff {} must disclose its trust / policy posture",
                    handoff.handoff_id
                ),
            ));
        }
        if !handoff.privacy_consequence_consistent() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::PrivacyConsequenceInconsistent,
                format!(
                    "handoff {} privacy consequence {} disagrees with its shared context",
                    handoff.handoff_id,
                    handoff.privacy_consequence.as_str()
                ),
            ));
        }
        if handoff.leaks_hidden_context() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::HiddenContextShareDetected,
                format!(
                    "handoff {} would exfiltrate raw code, README, ADR, or prompt context",
                    handoff.handoff_id
                ),
            ));
        }
        if handoff.ordinary_navigation_overshares() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::OrdinaryNavigationSharesContext,
                format!(
                    "handoff {} shares context as part of ordinary navigation",
                    handoff.handoff_id
                ),
            ));
        }
        if handoff.presents_unavailable_as_available() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::BlockedHandoffPresentedAvailable,
                format!(
                    "handoff {} is {} but is presented as an available action",
                    handoff.handoff_id,
                    handoff.policy_posture.as_str()
                ),
            ));
        }
        if handoff.bypasses_review() {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::RawBrowserOpenBypass,
                format!(
                    "handoff {} bypassed explicit handoff review",
                    handoff.handoff_id
                ),
            ));
        }
        if !handoff.raw_boundary_material_excluded {
            findings.push(BrowserHandoffValidationFinding::blocker(
                BrowserHandoffValidationKind::RawBoundaryMaterialPresent,
                format!(
                    "handoff {} retains raw boundary material",
                    handoff.handoff_id
                ),
            ));
        }
        if handoff.is_unavailable_narrowed() {
            findings.push(BrowserHandoffValidationFinding::warning(
                BrowserHandoffValidationKind::HandoffUnavailableNarrowed,
                format!(
                    "handoff {} is honestly disclosed as {} and narrows below stable",
                    handoff.handoff_id,
                    handoff.policy_posture.as_str()
                ),
            ));
        }
        if handoff.is_shared_context_blocked_narrowed() {
            findings.push(BrowserHandoffValidationFinding::warning(
                BrowserHandoffValidationKind::SharedContextBlockedNarrowed,
                format!(
                    "handoff {} honestly blocked a context share and narrows below stable",
                    handoff.handoff_id
                ),
            ));
        }
    }

    fn validate_exit_coverage(&self, findings: &mut Vec<BrowserHandoffValidationFinding>) {
        let covered: BTreeSet<HandoffSourceSurface> = self
            .handoffs
            .iter()
            .map(|handoff| handoff.source_surface)
            .collect();
        for required in HandoffSourceSurface::REQUIRED_EXITS {
            if !covered.contains(&required) {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::ExitCoverageMissing,
                    format!(
                        "no handoff routes the {} exit through a handoff object",
                        required.as_str()
                    ),
                ));
                break;
            }
        }
    }

    fn validate_projections(&self, findings: &mut Vec<BrowserHandoffValidationFinding>) {
        let present: BTreeSet<BrowserHandoffConsumerSurface> = self
            .consumer_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        for required in BrowserHandoffConsumerSurface::REQUIRED_RECONSTRUCTION {
            if !present.contains(&required) {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::RequiredReconstructionSurfaceMissing,
                    format!(
                        "no projection reconstructs the handoff on the {} surface",
                        required.as_str()
                    ),
                ));
                break;
            }
        }

        let known_ids: BTreeSet<&str> = self
            .handoffs
            .iter()
            .map(|handoff| handoff.handoff_id.as_str())
            .collect();

        for projection in &self.consumer_projections {
            if projection.packet_id_ref != self.packet_id {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::ConsumerProjectionPacketIdMismatch,
                    format!(
                        "surface {} references packet {}",
                        projection.surface.as_str(),
                        projection.packet_id_ref
                    ),
                ));
            }
            if !projection.preserves_required_flags() {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::ReconstructionProjectionDropsHandoff,
                    format!(
                        "surface {} drops a required handoff-reconstruction flag",
                        projection.surface.as_str()
                    ),
                ));
            }
            if projection.handoff_id_refs.is_empty() {
                findings.push(BrowserHandoffValidationFinding::blocker(
                    BrowserHandoffValidationKind::ReconstructionProjectionDropsHandoff,
                    format!(
                        "surface {} reconstructs no shared handoff",
                        projection.surface.as_str()
                    ),
                ));
            }
            for handoff_ref in &projection.handoff_id_refs {
                if !known_ids.contains(handoff_ref.as_str()) {
                    findings.push(BrowserHandoffValidationFinding::blocker(
                        BrowserHandoffValidationKind::ConsumerProjectionOrphanHandoffRef,
                        format!(
                            "surface {} references unknown handoff {}",
                            projection.surface.as_str(),
                            handoff_ref
                        ),
                    ));
                }
            }
        }

        self.validate_full_coverage(&known_ids, findings);
    }

    fn validate_full_coverage(
        &self,
        known_ids: &BTreeSet<&str>,
        findings: &mut Vec<BrowserHandoffValidationFinding>,
    ) {
        for projection in &self.consumer_projections {
            if !projection.surface.requires_full_coverage() {
                continue;
            }
            let reconstructed: BTreeSet<&str> = projection
                .handoff_id_refs
                .iter()
                .map(String::as_str)
                .collect();
            for handoff_id in known_ids {
                if !reconstructed.contains(handoff_id) {
                    findings.push(BrowserHandoffValidationFinding::blocker(
                        BrowserHandoffValidationKind::HistoryReconstructionDropsHandoff,
                        format!(
                            "surface {} drops handoff {} from its reconstruction",
                            projection.surface.as_str(),
                            handoff_id
                        ),
                    ));
                    break;
                }
            }
        }
    }
}

/// Support-export wrapper preserving the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Exported packet id.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when each handoff's identity is preserved across the export boundary.
    pub handoff_identity_preserved: bool,
    /// True when each handoff's return anchor is preserved across the boundary.
    pub return_anchor_preserved: bool,
    /// Exact packet preserved by the export.
    pub export_packet: BrowserHandoffPacket,
}

impl BrowserHandoffSupportExport {
    /// Returns true when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == BROWSER_HANDOFF_OBJECTS_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.handoff_identity_preserved
            && self.return_anchor_preserved
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in handoff export.
#[derive(Debug)]
pub enum BrowserHandoffArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export's packet failed validation.
    Validation(Vec<BrowserHandoffValidationFinding>),
    /// Support export wrapper is not export-safe.
    NotExportSafe,
}

impl fmt::Display for BrowserHandoffArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "browser handoff export parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "browser handoff export failed validation: {tokens}"
                )
            }
            Self::NotExportSafe => {
                write!(
                    formatter,
                    "browser handoff export wrapper is not export-safe"
                )
            }
        }
    }
}

impl Error for BrowserHandoffArtifactError {}

/// Returns the seeded stable handoff packet input.
pub fn seeded_stable_browser_handoff_input() -> BrowserHandoffPacketInput {
    seed::seeded_input()
}

/// Materializes the checked-in stable handoff packet.
///
/// # Errors
///
/// Returns an error when the seeded packet fails its own stable invariants.
pub fn current_stable_browser_handoff_packet(
) -> Result<BrowserHandoffPacket, BrowserHandoffArtifactError> {
    let packet = BrowserHandoffPacket::materialize(seeded_stable_browser_handoff_input());
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(BrowserHandoffArtifactError::Validation(findings))
    }
}

/// Reads and validates the checked-in stable support export.
///
/// # Errors
///
/// Returns an error when the checked artifact fails to parse, is not
/// export-safe, or its packet fails validation.
pub fn current_stable_browser_handoff_export(
) -> Result<BrowserHandoffSupportExport, BrowserHandoffArtifactError> {
    let export: BrowserHandoffSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/support_export.json"
    )))
    .map_err(BrowserHandoffArtifactError::SupportExport)?;
    let findings = export.export_packet.validate();
    if !findings.is_empty() {
        return Err(BrowserHandoffArtifactError::Validation(findings));
    }
    if !export.is_export_safe() {
        return Err(BrowserHandoffArtifactError::NotExportSafe);
    }
    Ok(export)
}

fn promotion_state_for(
    validation: &[BrowserHandoffValidationFinding],
) -> BrowserHandoffPromotionState {
    if validation
        .iter()
        .any(|finding| finding.severity == BrowserHandoffValidationSeverity::Blocker)
    {
        return BrowserHandoffPromotionState::BlocksStable;
    }
    if validation
        .iter()
        .any(|finding| finding.severity == BrowserHandoffValidationSeverity::Warning)
    {
        return BrowserHandoffPromotionState::NarrowedBelowStable;
    }
    BrowserHandoffPromotionState::Stable
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_url:")
                || lower.contains("raw_body:")
                || lower.contains("prompt_text:")
                || lower.contains("code_selection:")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

mod seed {
    use super::*;

    pub(super) const PACKET_ID: &str = "packet:browser_provider_console_handoff_objects:001";

    fn return_anchor(
        anchor_kind: ReturnAnchorKind,
        anchor_ref: &str,
        label: &str,
        follow_up_note: Option<&str>,
    ) -> ReturnAnchor {
        ReturnAnchor {
            anchor_kind,
            anchor_ref: anchor_ref.to_owned(),
            label: label.to_owned(),
            follow_up_note: follow_up_note.map(str::to_owned),
        }
    }

    fn docs_browser_handoff() -> BrowserHandoff {
        BrowserHandoff {
            handoff_id: "handoff:docs_browser:tokio-spawn-anchor".to_owned(),
            source_surface: HandoffSourceSurface::DocsBrowser,
            source_identity_ref: "docnode:mirror:tokio/runtime#spawn".to_owned(),
            source_class: DocsContractSourceClass::MirroredOfficialDocs,
            destination_class: HandoffDestinationClass::DocsOrPortalWeb,
            destination_ref: "destination:docs-portal:tokio/runtime#spawn".to_owned(),
            destination_reason: DocsContractBrowserHandoffReason::ExactAnchorUnavailableLocally,
            destination_reason_note:
                "the exact anchor is not in the local mirror; the upstream portal has it".to_owned(),
            privacy_consequence: DocsContractBrowserHandoffPrivacyConsequence::ScopedUrlOnly,
            shared_context: SharedContext {
                shares_resolved_destination_ref: true,
                ..SharedContext::NOTHING
            },
            trust_class: DocsContractTrustClass::SignedMirrorVerified,
            policy_posture: HandoffPolicyPosture::AllowedExplicit,
            policy_disclosure_note:
                "opens the signed vendor docs portal; only the resolved anchor crosses".to_owned(),
            return_anchor: return_anchor(
                ReturnAnchorKind::BackToDocsBrowser,
                "anchor:docs-browser:tokio-spawn-peek",
                "Back to the tokio::spawn peek",
                None,
            ),
            ordinary_navigation: true,
            user_initiated: false,
            offered_as_actionable: true,
            routed_through_handoff_review: true,
            raw_boundary_material_excluded: true,
        }
    }

    fn help_about_handoff() -> BrowserHandoff {
        BrowserHandoff {
            handoff_id: "handoff:help_about:product-docs-portal".to_owned(),
            source_surface: HandoffSourceSurface::HelpAbout,
            source_identity_ref: "surface:help_about:docs-link".to_owned(),
            source_class: DocsContractSourceClass::ProjectDocs,
            destination_class: HandoffDestinationClass::DocsOrPortalWeb,
            destination_ref: "destination:docs-portal:aureline/getting-started".to_owned(),
            destination_reason: DocsContractBrowserHandoffReason::SourceNotMirrored,
            destination_reason_note: "the full getting-started portal is not mirrored in-product"
                .to_owned(),
            privacy_consequence: DocsContractBrowserHandoffPrivacyConsequence::NoContextShared,
            shared_context: SharedContext::NOTHING,
            trust_class: DocsContractTrustClass::FirstPartyAuthoritative,
            policy_posture: HandoffPolicyPosture::AllowedExplicit,
            policy_disclosure_note:
                "opens the product's own docs portal; no workspace context crosses".to_owned(),
            return_anchor: return_anchor(
                ReturnAnchorKind::BackToHelpAbout,
                "anchor:help_about:about-panel",
                "Back to Help / About",
                Some("the getting-started guide opens in your browser"),
            ),
            ordinary_navigation: false,
            user_initiated: true,
            offered_as_actionable: true,
            routed_through_handoff_review: true,
            raw_boundary_material_excluded: true,
        }
    }

    fn ai_answer_handoff() -> BrowserHandoff {
        BrowserHandoff {
            handoff_id: "handoff:ai_answer:provider-search".to_owned(),
            source_surface: HandoffSourceSurface::AiAnswer,
            source_identity_ref: "explanation:ai_answer:where-is-the-runtime-built".to_owned(),
            source_class: DocsContractSourceClass::LiveExternalDocs,
            destination_class: HandoffDestinationClass::AiProviderWeb,
            destination_ref: "destination:ai-provider:search".to_owned(),
            destination_reason: DocsContractBrowserHandoffReason::UserRequestedOpenInBrowser,
            destination_reason_note:
                "the reader asked to continue the search on the provider's web surface".to_owned(),
            privacy_consequence: DocsContractBrowserHandoffPrivacyConsequence::QueryTermsDisclosed,
            shared_context: SharedContext {
                shares_resolved_destination_ref: true,
                shares_user_query_terms: true,
                ..SharedContext::NOTHING
            },
            trust_class: DocsContractTrustClass::LiveProviderHandoff,
            policy_posture: HandoffPolicyPosture::RequiresConfirmation,
            policy_disclosure_note:
                "shares only the typed query terms, disclosed; requires explicit confirmation"
                    .to_owned(),
            return_anchor: return_anchor(
                ReturnAnchorKind::BackToAiAnswer,
                "anchor:ai_answer:runtime-answer",
                "Back to the AI answer",
                None,
            ),
            ordinary_navigation: false,
            user_initiated: true,
            offered_as_actionable: true,
            routed_through_handoff_review: true,
            raw_boundary_material_excluded: true,
        }
    }

    fn provider_console_handoff() -> BrowserHandoff {
        BrowserHandoff {
            handoff_id: "handoff:provider_console:managed-admin".to_owned(),
            source_surface: HandoffSourceSurface::ProviderConsolePivot,
            source_identity_ref: "surface:provider_console:connected-provider".to_owned(),
            source_class: DocsContractSourceClass::LiveExternalDocs,
            destination_class: HandoffDestinationClass::ManagedAdminWeb,
            destination_ref: "destination:managed-admin:provider-settings".to_owned(),
            destination_reason: DocsContractBrowserHandoffReason::UserRequestedOpenInBrowser,
            destination_reason_note:
                "provider account management lives only on the hosted admin console".to_owned(),
            privacy_consequence: DocsContractBrowserHandoffPrivacyConsequence::IsolatedSession,
            shared_context: SharedContext {
                shares_resolved_destination_ref: true,
                ..SharedContext::NOTHING
            },
            trust_class: DocsContractTrustClass::LiveProviderHandoff,
            policy_posture: HandoffPolicyPosture::AllowedExplicit,
            policy_disclosure_note:
                "opens an isolated provider session that shares no prior workspace state".to_owned(),
            return_anchor: return_anchor(
                ReturnAnchorKind::BackToProviderPanel,
                "anchor:provider_console:connected-provider-panel",
                "Back to the connected-provider panel",
                Some("changes on the console sync back on your next refresh"),
            ),
            ordinary_navigation: false,
            user_initiated: true,
            offered_as_actionable: true,
            routed_through_handoff_review: true,
            raw_boundary_material_excluded: true,
        }
    }

    fn review_handoff() -> BrowserHandoff {
        BrowserHandoff {
            handoff_id: "handoff:review_surface:hosted-thread".to_owned(),
            source_surface: HandoffSourceSurface::ReviewSurface,
            source_identity_ref: "review-thread:pr-anchor".to_owned(),
            source_class: DocsContractSourceClass::LiveExternalDocs,
            destination_class: HandoffDestinationClass::CodeHostWeb,
            destination_ref: "destination:code-host:review-thread".to_owned(),
            destination_reason: DocsContractBrowserHandoffReason::ReviewThreadRequiresHostedView,
            destination_reason_note: "the full review thread requires the hosted review view"
                .to_owned(),
            privacy_consequence: DocsContractBrowserHandoffPrivacyConsequence::ScopedUrlOnly,
            shared_context: SharedContext {
                shares_resolved_destination_ref: true,
                ..SharedContext::NOTHING
            },
            trust_class: DocsContractTrustClass::LiveProviderHandoff,
            policy_posture: HandoffPolicyPosture::AllowedExplicit,
            policy_disclosure_note:
                "opens the hosted review thread; only the resolved thread ref crosses".to_owned(),
            return_anchor: return_anchor(
                ReturnAnchorKind::BackToReviewPanel,
                "anchor:review:review-panel",
                "Back to the review panel",
                None,
            ),
            ordinary_navigation: false,
            user_initiated: true,
            offered_as_actionable: true,
            routed_through_handoff_review: true,
            raw_boundary_material_excluded: true,
        }
    }

    pub(super) fn handoffs() -> Vec<BrowserHandoff> {
        vec![
            docs_browser_handoff(),
            help_about_handoff(),
            ai_answer_handoff(),
            provider_console_handoff(),
            review_handoff(),
        ]
    }

    fn projection(
        surface: BrowserHandoffConsumerSurface,
        handoff_id_refs: Vec<String>,
    ) -> BrowserHandoffConsumerProjection {
        BrowserHandoffConsumerProjection {
            surface,
            projection_ref: format!("projection:{}:{}", PACKET_ID, surface.as_str()),
            packet_id_ref: PACKET_ID.to_owned(),
            reuses_shared_handoff_object: true,
            preserves_destination_reason: true,
            preserves_return_anchor: true,
            preserves_privacy_consequence: true,
            handoff_id_refs,
        }
    }

    pub(super) fn projections() -> Vec<BrowserHandoffConsumerProjection> {
        let all_ids: Vec<String> = handoffs()
            .iter()
            .map(|handoff| handoff.handoff_id.clone())
            .collect();
        vec![
            // The browser companion shows the live handoff's reason and privacy.
            projection(
                BrowserHandoffConsumerSurface::BrowserCompanion,
                all_ids.clone(),
            ),
            // Help / About can reconstruct any prior handoff.
            projection(BrowserHandoffConsumerSurface::HelpAbout, all_ids.clone()),
            // The support export reconstructs every handoff so an export never
            // flattens a handoff into ordinary navigation.
            projection(
                BrowserHandoffConsumerSurface::SupportExport,
                all_ids.clone(),
            ),
            // Reopened docs history reconstructs every prior handoff.
            projection(BrowserHandoffConsumerSurface::DocsHistory, all_ids),
        ]
    }

    pub(super) fn seeded_input() -> BrowserHandoffPacketInput {
        BrowserHandoffPacketInput {
            packet_id: PACKET_ID.to_owned(),
            surface_label:
                "workflow:browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor:stable"
                    .to_owned(),
            generated_at: "2026-06-26T00:00:00Z".to_owned(),
            handoffs: handoffs(),
            consumer_projections: projections(),
            source_contract_refs: vec![
                BROWSER_HANDOFF_OBJECTS_SCHEMA_REF.to_owned(),
                BROWSER_HANDOFF_OBJECTS_DOC_REF.to_owned(),
                BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF.to_owned(),
                BROWSER_HANDOFF_OBJECTS_SUMMARY_REF.to_owned(),
                BROWSER_HANDOFF_OBJECTS_MATRIX_CONTRACT_REF.to_owned(),
                BROWSER_HANDOFF_OBJECTS_INTEGRATION_CONTRACT_REF.to_owned(),
            ],
            redaction_class_token: "metadata_safe_default".to_owned(),
        }
    }
}
