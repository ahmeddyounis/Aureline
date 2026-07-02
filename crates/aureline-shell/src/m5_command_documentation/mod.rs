//! Command-documentation surfaces, canonical examples, and alias/deprecation notes for every claimed M5
//! command family.
//!
//! The [frozen discoverability matrix][matrix] already binds each governed M5 last-mile command-discovery
//! surface — menu items, menu groups, context menus, command bars, keybinding resolver layers, conflict
//! review sheets, import-bridge rows, disabled-command explainers, leader/sequence help overlays, and
//! command-documentation surfaces — to one canonical command record, and freezes the required-label,
//! feature-family, discovery-channel, and downgrade-trigger vocabulary those surfaces project from. This
//! lane is the **command-documentation capstone** that certifies, for every one of those ten surface
//! families, that the surface publishes *documentation truth*: the same command id, primary label, aliases,
//! lifecycle / deprecation state, supported surfaces, invocation-schema summary, side-effect / risk class,
//! and result / rollback semantics that the shipped command record carries — with canonical examples and
//! replacement guidance that stay fresh across help, onboarding, migration, CLI/headless, and support
//! surfaces rather than drifting into a second naming system.
//!
//! For every surface family the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the surface publishes a **documentation record** with the stable command id, primary label, aliases,
//!   lifecycle / deprecation state, supported surfaces, invocation-schema summary, side-effect / risk
//!   class, and result / rollback semantics, plus canonical examples, that match the shipped command record
//!   ([`DocumentationRecordState`], acceptance criterion 1);
//! - help, onboarding, migration, CLI/headless, and support surfaces keep the **same canonical naming and
//!   replacement guidance** — no surface invents an alternate label or drifts on the deprecation /
//!   replacement command id ([`CrossSurfaceNamingState`], acceptance criterion 2);
//! - the surface's **canonical examples stay fresh and are never alias-only** — a stale example or an
//!   example that quotes only a deprecated alias is caught by proof freshness rather than surviving into a
//!   public/help surface ([`ExampleFreshnessState`], acceptance criterion 3);
//! - and the documentation packet is **copy-safe and diffable** so support bundles, docs/help, and
//!   migration packets can reconstruct the same command id and replacement guidance without a screenshot
//!   ([`DocExportParityState`], the copy-safe-introspection implementation requirement).
//!
//! Three records carry the truth:
//!
//! - the per-family **documentation row** ([`CommandDocRow`]): one row per [`M5CommandSurfaceFamily`] naming
//!   the canonical command binding it projects from, the required labels and feature families it exposes,
//!   the documentation-record fields it certifies, the parity cards it renders, the derivation anchors it
//!   derives from the shared record, the consumer surfaces it evaluated, its documentation-record /
//!   cross-surface-naming / example-freshness / doc-export posture, whether the same documentation survives
//!   headless/CLI execution, any active waiver, and a derived green/yellow/red [`CommandDocStatus`].
//! - the documentation **packet** ([`CommandDocPacket`]): the full set of rows with derived per-row status,
//!   aggregate green/yellow/red counts, the active waivers, the exact conformance causes
//!   ([`CommandDocCause`]), and the blocking findings the lane refuses to ship with.
//! - the documentation **dashboard** ([`CommandDocDashboard`]): a light projection the palette / help /
//!   onboarding / Support Center / CLI / migration tooling reads to auto-narrow a surface's
//!   documentation claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a surface
//! discloses a reduced documentation detail, a disclosed, waivered surface paraphrase, a disclosed partial
//! example refresh, or a disclosed partial doc export/capture; it drops to `red` if a surface ships a
//! missing or mismatched documentation record, drifts on canonical naming or replacement guidance, ships a
//! stale or alias-only example, cannot reconstruct its command id and replacement guidance from durable
//! evidence, loses the same documentation in a headless/CLI execution, or fails to certify all eight
//! documentation-record fields, all seven parity cards, all three derivation anchors, or every declared
//! consumer surface. That derivation is the auto-narrowing the acceptance criteria require, and the
//! doc-field, parity-card, derivation-anchor, and consumer-surface completeness checks are the conformance
//! lints that gate a stable documentation claim.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local paths,
//! raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary, counts,
//! refs, and short labels. The surface-family, canonical-command-binding, required-label, lifecycle-label,
//! preview-class, feature-family, consumer-surface, downgrade-trigger, and qualification vocabulary is
//! re-exported by reference from the already frozen [matrix], and every family's canonical command binding,
//! qualification, owner, required labels, lifecycle label, feature families, declared consumer surfaces, and
//! applicable downgrade triggers are pulled straight from that matrix's seeded packet, so this lane mints no
//! parallel command vocabulary and cannot certify a surface the matrix does not anchor. Only the
//! documentation-specific vocabulary ([`M5CommandDocDimension`], [`M5CommandDocField`],
//! [`M5CommandParityCard`], [`M5DocDerivationAnchor`], [`CommandDocStatus`], [`DocumentationRecordState`],
//! [`CrossSurfaceNamingState`], [`ExampleFreshnessState`], [`DocExportParityState`], [`CommandDocWaiver`],
//! [`CommandDocCause`], [`CommandDocFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix as matrix;

pub use matrix::{
    M5CanonicalCommandBinding, M5CommandSurfaceFamily, M5DisabledReasonMode,
    M5DiscoverabilityDowngradeTrigger, M5DiscoveryChannel, M5FeatureFamily, M5LifecycleLabel,
    M5PreviewClass, M5RequiredLabel, M5SurfaceQualificationClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_command_documentation_packet,
    seeded_m5_command_documentation_packet_context_menu_naming_drift_blocked,
    seeded_m5_command_documentation_packet_documentation_surface_stale_example_blocked,
    seeded_m5_command_documentation_packet_explainer_headless_parity_lost_blocked,
    seeded_m5_command_documentation_packet_import_bridge_capture_absent_blocked,
    seeded_m5_command_documentation_packet_menu_item_record_mismatch_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_COMMAND_DOC_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_COMMAND_DOC_SHARED_CONTRACT_REF: &str = "commands:m5_command_documentation:v1";

/// Stable record kind for [`CommandDocPacket`] payloads.
pub const M5_COMMAND_DOC_PACKET_RECORD_KIND: &str =
    "commands_m5_command_documentation_packet_record";

/// Stable record kind for [`CommandDocDashboard`] payloads.
pub const M5_COMMAND_DOC_DASHBOARD_RECORD_KIND: &str =
    "commands_m5_command_documentation_dashboard_record";

/// Stable record kind for [`CommandDocSupportExport`] payloads.
pub const M5_COMMAND_DOC_SUPPORT_EXPORT_RECORD_KIND: &str =
    "commands_m5_command_documentation_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_COMMAND_DOC_PACKET_ID: &str = "m5-command-documentation:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_COMMAND_DOC_DASHBOARD_ID: &str = "m5-command-documentation-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_COMMAND_DOC_SUPPORT_EXPORT_ID: &str = "support-export:m5-command-documentation:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_COMMAND_DOC_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-command-documentation.schema.json";

/// Published markdown report ref reviewers reopen the documentation proof from.
pub const M5_COMMAND_DOC_PUBLISHED_REPORT_REF: &str =
    "artifacts/commands/m5-command-documentation.md";

/// Published documentation-packet artifact ref.
pub const M5_COMMAND_DOC_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-command-documentation-proof/packet.json";

/// Published documentation-dashboard artifact ref.
pub const M5_COMMAND_DOC_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-command-documentation-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_COMMAND_DOC_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-command-documentation-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_COMMAND_DOC_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-command-documentation-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_COMMAND_DOC_PUBLISHED_DOC_REF: &str =
    "docs/commands/m5_command_documentation_contract.md";

/// Repo-relative ref to the frozen discoverability boundary schema.
pub const M5_COMMAND_DOC_MATRIX_SCHEMA_REF: &str = matrix::M5_DISCOVERABILITY_SCHEMA_REF;

/// Frozen discoverability contract doc this proof mirrors.
pub const M5_COMMAND_DOC_MATRIX_DOC_REF: &str = matrix::M5_DISCOVERABILITY_DOC_REF;

/// Canonical command-descriptor schema every documentation surface projects from.
pub const M5_COMMAND_DOC_COMMAND_DESCRIPTOR_REF: &str =
    matrix::M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF;

/// Every command-surface family the certification must cover, in canonical order. A certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_SURFACE_FAMILIES: [M5CommandSurfaceFamily; 10] = M5CommandSurfaceFamily::ALL;

/// Every documentation dimension each family row certifies, in canonical order.
pub const REQUIRED_DOC_DIMENSIONS: [M5CommandDocDimension; 4] = M5CommandDocDimension::ALL;

/// Every documentation-record field each family row must publish, in canonical order.
pub const REQUIRED_DOC_FIELDS: [M5CommandDocField; 8] = M5CommandDocField::ALL;

/// Every parity card each family row must render, in canonical order.
pub const REQUIRED_PARITY_CARDS: [M5CommandParityCard; 7] = M5CommandParityCard::ALL;

/// Every derivation anchor each family row must derive from the shared record, in canonical order.
pub const REQUIRED_DERIVATION_ANCHORS: [M5DocDerivationAnchor; 3] = M5DocDerivationAnchor::ALL;

/// One of the four documentation dimensions each surface-family row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a claimed
/// M5 command surface publish documentation truth: the documentation record matches the shipped command
/// record; naming and replacement guidance stay stable across surfaces; canonical examples stay fresh and
/// are never alias-only; and the documentation packet reconstructs the command id and replacement guidance
/// from durable evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandDocDimension {
    /// The documentation record matches the shipped command record.
    DocumentationRecord,
    /// Naming and replacement guidance stay stable across surfaces.
    CrossSurfaceNaming,
    /// Canonical examples stay fresh and are never alias-only.
    ExampleFreshness,
    /// The documentation packet reconstructs the command id and replacement guidance from durable evidence.
    DocExportParity,
}

impl M5CommandDocDimension {
    /// Every documentation dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DocumentationRecord,
        Self::CrossSurfaceNaming,
        Self::ExampleFreshness,
        Self::DocExportParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocumentationRecord => "documentation_record",
            Self::CrossSurfaceNaming => "cross_surface_naming",
            Self::ExampleFreshness => "example_freshness",
            Self::DocExportParity => "doc_export_parity",
        }
    }
}

/// One of the eight fields a command-documentation record must publish for a claimed M5 command.
///
/// These are the exact fields the implementation requirements name: the stable command id, the primary
/// label, the aliases, the lifecycle / deprecation state, the supported surfaces, the invocation-schema
/// summary, the side-effect / risk class, and the result / rollback semantics. A record that publishes
/// fewer cannot honestly claim documentation truth and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandDocField {
    /// The stable command id.
    CommandId,
    /// The canonical primary label.
    PrimaryLabel,
    /// The command aliases (including deprecated aliases).
    Aliases,
    /// The lifecycle / deprecation state.
    LifecycleState,
    /// The surfaces the command is supported on.
    SupportedSurfaces,
    /// The invocation-schema summary (arguments / options).
    InvocationSchemaSummary,
    /// The side-effect / risk class.
    SideEffectRiskClass,
    /// The result / rollback semantics.
    ResultRollbackSemantics,
}

impl M5CommandDocField {
    /// Every documentation-record field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CommandId,
        Self::PrimaryLabel,
        Self::Aliases,
        Self::LifecycleState,
        Self::SupportedSurfaces,
        Self::InvocationSchemaSummary,
        Self::SideEffectRiskClass,
        Self::ResultRollbackSemantics,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandId => "command_id",
            Self::PrimaryLabel => "primary_label",
            Self::Aliases => "aliases",
            Self::LifecycleState => "lifecycle_state",
            Self::SupportedSurfaces => "supported_surfaces",
            Self::InvocationSchemaSummary => "invocation_schema_summary",
            Self::SideEffectRiskClass => "side_effect_risk_class",
            Self::ResultRollbackSemantics => "result_rollback_semantics",
        }
    }
}

/// One of the seven parity cards a documentation surface must render, showing how the same command appears
/// across every reach.
///
/// These are the parity surfaces the implementation requirements name — the same command shown in menus,
/// buttons, the palette, CLI/headless, AI tools, recipes, and voice/companion hints — so a reader sees one
/// canonical command semantics rather than a per-surface reinvention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandParityCard {
    /// How the command appears in a menu.
    Menu,
    /// How the command appears as a button / toolbar affordance.
    Button,
    /// How the command appears as a palette row.
    Palette,
    /// How the command appears in CLI / headless help.
    CliHeadless,
    /// How the command appears as an AI automation tool.
    AiTool,
    /// How the command appears in a recipe / automation script.
    Recipe,
    /// How the command appears as a voice / companion hint.
    VoiceCompanionHint,
}

impl M5CommandParityCard {
    /// Every parity card, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Menu,
        Self::Button,
        Self::Palette,
        Self::CliHeadless,
        Self::AiTool,
        Self::Recipe,
        Self::VoiceCompanionHint,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Menu => "menu",
            Self::Button => "button",
            Self::Palette => "palette",
            Self::CliHeadless => "cli_headless",
            Self::AiTool => "ai_tool",
            Self::Recipe => "recipe",
            Self::VoiceCompanionHint => "voice_companion_hint",
        }
    }
}

/// One of the three derivation anchors a documentation surface must derive from the shared command record
/// rather than duplicate by hand.
///
/// These are the anchors the implementation requirements name: the docs/help anchor, the shortcut notation,
/// and the accessibility narration hint. Each must be derived from the same command record so the surfaces
/// cannot drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocDerivationAnchor {
    /// The docs/help anchor the surface reopens the command from.
    DocsHelpAnchor,
    /// The shortcut notation shown for the command.
    ShortcutNotation,
    /// The accessibility narration hint announced for the command.
    AccessibilityNarrationHint,
}

impl M5DocDerivationAnchor {
    /// Every derivation anchor, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DocsHelpAnchor,
        Self::ShortcutNotation,
        Self::AccessibilityNarrationHint,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelpAnchor => "docs_help_anchor",
            Self::ShortcutNotation => "shortcut_notation",
            Self::AccessibilityNarrationHint => "accessibility_narration_hint",
        }
    }
}

/// The derived documentation light a command surface carries.
///
/// `green` means the surface publishes a documentation record matching the shipped command record across
/// all eight fields, keeps canonical naming and replacement guidance stable, keeps its canonical examples
/// fresh and never alias-only, and reconstructs its command id and replacement guidance from durable
/// evidence — across every declared consumer surface, with the same documentation surviving headless/CLI
/// execution. `yellow` is a disclosed narrowing. `red` is blocked and may not keep a documentation claim
/// until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandDocStatus {
    /// Full standing: all four documentation dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl CommandDocStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the documentation surface publishes its documentation record.
///
/// `command_record_examples_and_lifecycle_certified` means the surface publishes the command id, primary
/// label, aliases, lifecycle / deprecation state, supported surfaces, invocation-schema summary,
/// side-effect / risk class, and result / rollback semantics — plus canonical examples — matching the
/// shipped command record. `disclosed_reduced_doc_detail` means a constrained surface folds the
/// invocation-schema summary and side-effect detail into an expandable section while still naming the
/// command id, primary label, and lifecycle / deprecation truth (a yellow narrowing).
/// `doc_record_missing_or_mismatched` means the record is absent or disagrees with the shipped command
/// record — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentationRecordState {
    /// The documentation record and canonical examples match the shipped command record.
    CommandRecordExamplesAndLifecycleCertified,
    /// A constrained surface takes a disclosed reduced documentation detail.
    DisclosedReducedDocDetail,
    /// The documentation record is missing or mismatched — a blocker.
    DocRecordMissingOrMismatched,
}

impl DocumentationRecordState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandRecordExamplesAndLifecycleCertified => {
                "command_record_examples_and_lifecycle_certified"
            }
            Self::DisclosedReducedDocDetail => "disclosed_reduced_doc_detail",
            Self::DocRecordMissingOrMismatched => "doc_record_missing_or_mismatched",
        }
    }

    /// `true` when the documentation record is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CommandRecordExamplesAndLifecycleCertified)
    }

    /// `true` when the surface took a disclosed reduced-documentation-detail narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedDocDetail)
    }
}

/// How the surface keeps canonical naming and replacement guidance stable across help / onboarding /
/// migration / CLI / support surfaces.
///
/// `canonical_naming_and_replacement_stable` means every surface uses the canonical primary label and, for
/// a deprecated command, the same replacement command id. `disclosed_surface_paraphrase` means one
/// constrained surface renders a disclosed, waivered short paraphrase of the canonical label while still
/// pointing at the canonical command id and replacement guidance (a yellow narrowing that **requires an
/// active waiver**). `naming_or_replacement_drifted` means a surface invented an alternate label or drifted
/// on the replacement guidance — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossSurfaceNamingState {
    /// Canonical naming and replacement guidance are certified stable.
    CanonicalNamingAndReplacementStable,
    /// One constrained surface renders a disclosed, waivered short paraphrase.
    DisclosedSurfaceParaphrase,
    /// A surface invented an alternate label or drifted on replacement guidance — a blocker.
    NamingOrReplacementDrifted,
}

impl CrossSurfaceNamingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalNamingAndReplacementStable => "canonical_naming_and_replacement_stable",
            Self::DisclosedSurfaceParaphrase => "disclosed_surface_paraphrase",
            Self::NamingOrReplacementDrifted => "naming_or_replacement_drifted",
        }
    }

    /// `true` when cross-surface naming is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CanonicalNamingAndReplacementStable)
    }

    /// `true` when the surface took a disclosed surface-paraphrase narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedSurfaceParaphrase)
    }
}

/// How the surface keeps its canonical examples fresh and never alias-only.
///
/// `canonical_examples_fresh_and_not_alias_only` means every canonical example is current with the shipped
/// command record and quotes the canonical command id rather than only a deprecated alias.
/// `disclosed_partial_example_refresh` means one example slice still awaits refresh, the gap is disclosed,
/// and the stale slice is flagged rather than presented as current (a yellow narrowing).
/// `stale_or_alias_only_example_shipped` means a stale example or an alias-only example reached a
/// public/help surface — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExampleFreshnessState {
    /// Canonical examples are fresh and never alias-only.
    CanonicalExamplesFreshAndNotAliasOnly,
    /// One example slice takes a disclosed partial example refresh.
    DisclosedPartialExampleRefresh,
    /// A stale or alias-only example reached a public/help surface — a blocker.
    StaleOrAliasOnlyExampleShipped,
}

impl ExampleFreshnessState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalExamplesFreshAndNotAliasOnly => {
                "canonical_examples_fresh_and_not_alias_only"
            }
            Self::DisclosedPartialExampleRefresh => "disclosed_partial_example_refresh",
            Self::StaleOrAliasOnlyExampleShipped => "stale_or_alias_only_example_shipped",
        }
    }

    /// `true` when example freshness is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CanonicalExamplesFreshAndNotAliasOnly)
    }

    /// `true` when the surface took a disclosed partial-example-refresh narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialExampleRefresh)
    }
}

/// How the documentation packet reconstructs the command id and replacement guidance.
///
/// `command_id_and_replacement_reconstructable` means a support bundle, doc, or migration packet can
/// reconstruct the command id and the replacement / deprecation guidance from a durable, copy-safe,
/// diffable export without a screenshot. `disclosed_partial_capture` means one legacy export captures the
/// command id and replacement guidance but not the full alias list, while still disclosing the gap (a
/// yellow narrowing). `doc_truth_absent_from_capture` means the command id or replacement guidance is
/// absent from durable evidence — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocExportParityState {
    /// Command id and replacement guidance are reconstructable from durable evidence.
    CommandIdAndReplacementReconstructable,
    /// One legacy export takes a disclosed partial capture.
    DisclosedPartialCapture,
    /// The command id or replacement guidance is absent from durable evidence — a blocker.
    DocTruthAbsentFromCapture,
}

impl DocExportParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandIdAndReplacementReconstructable => {
                "command_id_and_replacement_reconstructable"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::DocTruthAbsentFromCapture => "doc_truth_absent_from_capture",
        }
    }

    /// `true` when doc export parity is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CommandIdAndReplacementReconstructable)
    }

    /// `true` when the surface took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather than
/// blocked — never lets a mismatched record, a naming drift, a stale example, or an uncapturable command id
/// hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDocWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The surface family the waiver applies to.
    pub surface_family: M5CommandSurfaceFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl CommandDocWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a surface family's documentation.
///
/// The trigger token mirrors the frozen [`M5DiscoverabilityDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDocCause {
    /// The surface family the cause applies to.
    pub surface_family: M5CommandSurfaceFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5DiscoverabilityDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is a
    /// blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl CommandDocCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One surface family, certified across its documentation-record, cross-surface-naming, example-freshness,
/// and doc-export dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDocRow {
    /// The surface family being certified.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short reviewer-facing family label.
    pub surface_label: String,
    /// Qualification class the matrix earned for the surface. Pulled from the matrix.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this surface's documentation governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The canonical command-record binding this surface projects from. Pulled from the matrix.
    pub canonical_command_binding: M5CanonicalCommandBinding,
    /// The pinned lifecycle / deprecation label. Pulled from the canonical command binding.
    pub lifecycle_label: M5LifecycleLabel,
    /// Mandatory labels this surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5RequiredLabel>,
    /// M5 feature families whose commands this surface documents. Pulled from the matrix.
    pub feature_families: Vec<M5FeatureFamily>,
    /// The documentation-record fields this row publishes (must be all eight).
    pub certified_doc_fields: Vec<M5CommandDocField>,
    /// The parity cards this row renders (must be all seven).
    pub certified_parity_cards: Vec<M5CommandParityCard>,
    /// The derivation anchors this row derives from the shared record (must be all three).
    pub certified_derivation_anchors: Vec<M5DocDerivationAnchor>,
    /// Consumer surfaces the matrix declares the surface must project to.
    pub required_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Documentation-record posture.
    pub documentation_record: DocumentationRecordState,
    /// Cross-surface-naming posture.
    pub cross_surface_naming: CrossSurfaceNamingState,
    /// Example-freshness posture.
    pub example_freshness: ExampleFreshnessState,
    /// Doc-export-parity posture.
    pub doc_export_parity: DocExportParityState,
    /// `true` when the same documentation survives a headless / CLI execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the surface. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Active waiver, when a disclosed surface paraphrase is in force.
    pub active_waiver: Option<CommandDocWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: CommandDocStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<CommandDocCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl CommandDocRow {
    /// `true` when the row certified every consumer surface the matrix declares for the surface — no
    /// declared surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when the row publishes every one of the eight documentation-record fields — the structural
    /// proof that the documentation record is complete.
    pub fn doc_fields_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_doc_fields
            .iter()
            .map(|field| field.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_DOC_FIELDS
            .iter()
            .map(|field| field.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when the row renders every one of the seven parity cards — the structural proof that the same
    /// command appears identically across every reach.
    pub fn parity_cards_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_parity_cards
            .iter()
            .map(|card| card.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_PARITY_CARDS
            .iter()
            .map(|card| card.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when the row derives every one of the three derivation anchors from the shared record — the
    /// structural proof that docs/help anchors, shortcut notation, and narration hints are not duplicated
    /// by hand.
    pub fn derivation_anchors_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_derivation_anchors
            .iter()
            .map(|anchor| anchor.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_DERIVATION_ANCHORS
            .iter()
            .map(|anchor| anchor.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.doc_fields_complete() {
            return true;
        }
        if !self.parity_cards_complete() {
            return true;
        }
        if !self.derivation_anchors_complete() {
            return true;
        }
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.documentation_record,
            DocumentationRecordState::DocRecordMissingOrMismatched
        ) {
            return true;
        }
        if matches!(
            self.cross_surface_naming,
            CrossSurfaceNamingState::NamingOrReplacementDrifted
        ) {
            return true;
        }
        if matches!(
            self.example_freshness,
            ExampleFreshnessState::StaleOrAliasOnlyExampleShipped
        ) {
            return true;
        }
        if matches!(
            self.doc_export_parity,
            DocExportParityState::DocTruthAbsentFromCapture
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.documentation_record.is_disclosed_narrowing()
            || self.cross_surface_naming.is_disclosed_narrowing()
            || self.example_freshness.is_disclosed_narrowing()
            || self.doc_export_parity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the documentation posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> CommandDocStatus {
        if self.has_hard_blocker() {
            CommandDocStatus::Red
        } else if self.has_narrowing() {
            CommandDocStatus::Yellow
        } else {
            CommandDocStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (documentation record,
    /// cross-surface naming, example freshness, doc export, then structural completeness and headless
    /// parity).
    pub fn recompute_causes(&self) -> Vec<CommandDocCause> {
        let mut causes = Vec::new();
        match self.documentation_record {
            DocumentationRecordState::CommandRecordExamplesAndLifecycleCertified => {}
            DocumentationRecordState::DisclosedReducedDocDetail => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::LifecycleOrDeprecationHidden,
                    disclosed: true,
                    detail: "On a constrained surface the documentation record takes a disclosed reduced \
                             detail — the invocation-schema summary and side-effect / risk detail are \
                             folded into an expandable section while the command id, primary label, \
                             aliases, and lifecycle / deprecation truth stay visible — so the record is \
                             narrowed and disclosed rather than missing or mismatched."
                        .to_owned(),
                });
            }
            DocumentationRecordState::DocRecordMissingOrMismatched => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::LifecycleOrDeprecationHidden,
                    disclosed: false,
                    detail: "The documentation record is absent or disagrees with the shipped command \
                             record — the lifecycle / deprecation state, aliases, or side-effect class \
                             shown does not match the command descriptor — so a reader cannot trust the \
                             documented command truth."
                        .to_owned(),
                });
            }
        }
        match self.cross_surface_naming {
            CrossSurfaceNamingState::CanonicalNamingAndReplacementStable => {}
            CrossSurfaceNamingState::DisclosedSurfaceParaphrase => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: true,
                    detail: "One constrained surface renders a disclosed, waivered short paraphrase of the \
                             canonical label while still pointing at the canonical command id and the same \
                             replacement guidance — so the naming is narrowed and disclosed rather than an \
                             invented alternate label."
                        .to_owned(),
                });
            }
            CrossSurfaceNamingState::NamingOrReplacementDrifted => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: false,
                    detail: "A help / onboarding / migration / CLI / support surface invented an alternate \
                             label for the command or drifted on the deprecation / replacement command id, \
                             so the same command reads with different names or replacement guidance \
                             depending on where it is reached."
                        .to_owned(),
                });
            }
        }
        match self.example_freshness {
            ExampleFreshnessState::CanonicalExamplesFreshAndNotAliasOnly => {}
            ExampleFreshnessState::DisclosedPartialExampleRefresh => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One canonical-example slice takes a disclosed partial refresh — the stale \
                             slice is flagged and scheduled for refresh rather than presented as current — \
                             so the example freshness is narrowed and disclosed rather than shipping a \
                             stale or alias-only example unnoticed."
                        .to_owned(),
                });
            }
            ExampleFreshnessState::StaleOrAliasOnlyExampleShipped => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "A stale canonical example, or an example quoting only a deprecated alias \
                             instead of the canonical command id, reached a public/help surface, so a \
                             reader could copy an out-of-date or alias-only invocation."
                        .to_owned(),
                });
            }
        }
        match self.doc_export_parity {
            DocExportParityState::CommandIdAndReplacementReconstructable => {}
            DocExportParityState::DisclosedPartialCapture => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One legacy documentation export takes a disclosed partial capture — the export \
                             captures the command id and replacement guidance but not the full alias list, \
                             while still disclosing the gap — so the copy-safe export parity is narrowed \
                             and disclosed rather than absent."
                        .to_owned(),
                });
            }
            DocExportParityState::DocTruthAbsentFromCapture => {
                causes.push(CommandDocCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "The command id or the replacement / deprecation guidance is absent from the \
                             durable, diffable documentation export, so a support bundle, doc, or migration \
                             packet cannot reconstruct the documented command truth without a screenshot."
                        .to_owned(),
                });
            }
        }
        if !self.doc_fields_complete() {
            causes.push(CommandDocCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::CommandIdMissing,
                disclosed: false,
                detail: "The documentation record does not publish all eight fields — command id, primary \
                         label, aliases, lifecycle state, supported surfaces, invocation-schema summary, \
                         side-effect / risk class, and result / rollback semantics — so the documented \
                         command truth is incomplete."
                    .to_owned(),
            });
        }
        if !self.parity_cards_complete() {
            causes.push(CommandDocCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "The surface does not render all seven parity cards — menu, button, palette, \
                         CLI/headless, AI tool, recipe, and voice/companion hint — so the same command \
                         could read differently depending on the reach."
                    .to_owned(),
            });
        }
        if !self.derivation_anchors_complete() {
            causes.push(CommandDocCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                disclosed: false,
                detail: "The surface does not derive all three anchors — docs/help anchor, shortcut \
                         notation, and accessibility narration hint — from the shared command record, so a \
                         hand-duplicated anchor could drift from the canonical record."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(CommandDocCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "A headless / CLI execution of this surface lost the shared documentation, so the \
                         same command documents a different record, naming, or example depending on how it \
                         is reached."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed surface paraphrase may only stay yellow (rather than red) when a waiver discloses it —
    /// paraphrasing the canonical label is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.cross_surface_naming,
            CrossSurfaceNamingState::DisclosedSurfaceParaphrase
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<CommandDocFinding> {
        let mut findings = Vec::new();
        let family = self.surface_family.as_str().to_owned();

        if !self.doc_fields_complete() {
            findings.push(CommandDocFinding::DocFieldsIncomplete {
                family: family.clone(),
            });
        }
        if !self.parity_cards_complete() {
            findings.push(CommandDocFinding::ParityCardsIncomplete {
                family: family.clone(),
            });
        }
        if !self.derivation_anchors_complete() {
            findings.push(CommandDocFinding::DerivationAnchorsIncomplete {
                family: family.clone(),
            });
        }
        if !self.consumer_surfaces_complete() {
            findings.push(CommandDocFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(CommandDocFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.documentation_record,
            DocumentationRecordState::DocRecordMissingOrMismatched
        ) {
            findings.push(CommandDocFinding::DocumentationRecordBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.cross_surface_naming,
            CrossSurfaceNamingState::NamingOrReplacementDrifted
        ) {
            findings.push(CommandDocFinding::CrossSurfaceNamingBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.example_freshness,
            ExampleFreshnessState::StaleOrAliasOnlyExampleShipped
        ) {
            findings.push(CommandDocFinding::ExampleFreshnessBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.doc_export_parity,
            DocExportParityState::DocTruthAbsentFromCapture
        ) {
            findings.push(CommandDocFinding::DocExportBroken {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, CommandDocStatus::Green) && !self.has_reason() {
            findings.push(CommandDocFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(CommandDocFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.surface_family != self.surface_family {
                findings.push(CommandDocFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(CommandDocFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(CommandDocFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(CommandDocFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} record={} naming={} examples={} export={} headless={} lifecycle={} fields={} cards={} surfaces={} waiver={}",
            self.surface_family.as_str(),
            self.derived_status.as_str(),
            self.documentation_record.as_str(),
            self.cross_surface_naming.as_str(),
            self.example_freshness.as_str(),
            self.doc_export_parity.as_str(),
            self.headless_parity_preserved,
            self.lifecycle_label.as_str(),
            self.certified_doc_fields.len(),
            self.certified_parity_cards.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the documentation certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum CommandDocFinding {
    /// A surface family has no documentation row.
    SurfaceFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not publish every documentation-record field.
    DocFieldsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not render every parity card.
    ParityCardsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not derive every derivation anchor from the shared record.
    DerivationAnchorsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless / CLI execution lost the shared documentation.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// The documentation record is missing or mismatched.
    DocumentationRecordBroken {
        /// The family token.
        family: String,
    },
    /// A surface invented an alternate label or drifted on replacement guidance.
    CrossSurfaceNamingBroken {
        /// The family token.
        family: String,
    },
    /// A stale or alias-only example reached a public/help surface.
    ExampleFreshnessBroken {
        /// The family token.
        family: String,
    },
    /// The command id or replacement guidance is absent from the durable documentation export.
    DocExportBroken {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared conformance causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl CommandDocFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::SurfaceFamilyMissing { .. } => "surface_family_missing",
            Self::DocFieldsIncomplete { .. } => "doc_fields_incomplete",
            Self::ParityCardsIncomplete { .. } => "parity_cards_incomplete",
            Self::DerivationAnchorsIncomplete { .. } => "derivation_anchors_incomplete",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::DocumentationRecordBroken { .. } => "documentation_record_broken",
            Self::CrossSurfaceNamingBroken { .. } => "cross_surface_naming_broken",
            Self::ExampleFreshnessBroken { .. } => "example_freshness_broken",
            Self::DocExportBroken { .. } => "doc_export_broken",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::SurfaceFamilyMissing { family }
            | Self::DocFieldsIncomplete { family }
            | Self::ParityCardsIncomplete { family }
            | Self::DerivationAnchorsIncomplete { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::DocumentationRecordBroken { family }
            | Self::CrossSurfaceNamingBroken { family }
            | Self::ExampleFreshnessBroken { family }
            | Self::DocExportBroken { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The documentation packet shared by the palette / help / onboarding / Support Center / CLI / migration
/// tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDocPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen discoverability matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen discoverability boundary schema.
    pub matrix_schema_ref: String,
    /// Frozen discoverability contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// Canonical command-descriptor schema every documentation surface projects from.
    pub command_descriptor_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four documentation dimensions every family row certifies.
    pub required_doc_dimensions: Vec<String>,
    /// The eight documentation-record fields every family row must publish.
    pub required_doc_fields: Vec<String>,
    /// The seven parity cards every family row must render.
    pub required_parity_cards: Vec<String>,
    /// The three derivation anchors every family row must derive.
    pub required_derivation_anchors: Vec<String>,
    /// The ten surface families the certification must cover.
    pub required_surface_families: Vec<String>,
    /// Per-family documentation rows, in canonical order.
    pub rows: Vec<CommandDocRow>,
    /// Surface families certified, in canonical (sorted) order.
    pub covered_surface_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-conformance) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked — the stable-claim gate.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<CommandDocWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<CommandDocCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<CommandDocFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Command / documentation automation refs that consume this packet to auto-narrow a surface.
    pub command_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help / migration refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published documentation-packet ref.
    pub published_packet_ref: String,
    /// Published documentation-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl CommandDocPacket {
    /// Returns the documentation row for `family`, if present.
    pub fn row(&self, family: M5CommandSurfaceFamily) -> Option<&CommandDocRow> {
        self.rows.iter().find(|row| row.surface_family == family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.surface_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.conformance_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.surface_family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light documentation dashboard the command automation consumes.
    pub fn dashboard(&self) -> CommandDocDashboard {
        CommandDocDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 command-documentation packet serializes")
    }

    /// Deterministic, machine-readable documentation CSV: one row per surface family naming its status, the
    /// four documentation postures, headless parity, the lifecycle label, the doc-field / parity-card
    /// counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,status,documentation_record,cross_surface_naming,example_freshness,doc_export_parity,headless_parity,lifecycle,doc_fields,parity_cards,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.derived_status.as_str(),
                row.documentation_record.as_str(),
                row.cross_surface_naming.as_str(),
                row.example_freshness.as_str(),
                row.doc_export_parity.as_str(),
                row.headless_parity_preserved,
                row.lifecycle_label.as_str(),
                row.certified_doc_fields.len(),
                row.certified_parity_cards.len(),
                row.evaluated_consumer_surfaces.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 command documentation: canonical command-record docs, parity cards, fresh examples, and copy-safe alias/deprecation export across every claimed M5 command surface\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_command_documentation`](../../crates/aureline-shell/src/m5_command_documentation/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_documentation -- markdown > \\\n  artifacts/commands/m5-command-documentation.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!(
            "- Required documentation dimensions: {}\n",
            self.required_doc_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Documentation-record fields published: {}\n",
            self.required_doc_fields
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Parity cards rendered: {}\n",
            self.required_parity_cards
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Derivation anchors: {}\n",
            self.required_derivation_anchors
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Surface families certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full conformance): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable (stable-claim gate): `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Documentation rows\n\n");
        out.push_str(
            "| Surface family | Status | Documentation record | Cross-surface naming | Example freshness | Doc export | Lifecycle | Headless | Waiver |\n\
             | -------------- | ------ | -------------------- | -------------------- | ----------------- | ---------- | --------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.documentation_record.as_str(),
                row.cross_surface_naming.as_str(),
                row.example_freshness.as_str(),
                row.doc_export_parity.as_str(),
                row.lifecycle_label.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&CommandDocRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, CommandDocStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 command surface publishes a documentation record matching the shipped command record, keeps canonical naming and replacement guidance stable, keeps its canonical examples fresh and never alias-only, and reconstructs its command id and replacement guidance from durable evidence across every declared consumer surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.surface_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact conformance causes\n\n");
        if self.conformance_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.conformance_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.surface_family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.surface_family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_documentation -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_command_documentation_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light documentation dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDocDashboardRow {
    /// The surface family.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short family label.
    pub surface_label: String,
    /// Qualification class earned by the surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Derived green/yellow/red status.
    pub status: CommandDocStatus,
    /// The pinned lifecycle / deprecation label.
    pub lifecycle_label: M5LifecycleLabel,
    /// Number of documentation-record fields published.
    pub certified_doc_field_count: usize,
    /// Number of parity cards rendered.
    pub certified_parity_card_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Documentation-record posture.
    pub documentation_record: DocumentationRecordState,
    /// Cross-surface-naming posture.
    pub cross_surface_naming: CrossSurfaceNamingState,
    /// Example-freshness posture.
    pub example_freshness: ExampleFreshnessState,
    /// Doc-export-parity posture.
    pub doc_export_parity: DocExportParityState,
    /// `true` when headless / CLI parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light documentation dashboard the palette / help / onboarding / Support Center / CLI / migration
/// tooling reads to auto-narrow a surface's documentation claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDocDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<CommandDocDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Command / documentation automation refs that consume the dashboard.
    pub command_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl CommandDocDashboard {
    /// Projects the dashboard from a documentation packet.
    pub fn from_packet(packet: &CommandDocPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| CommandDocDashboardRow {
                surface_family: row.surface_family,
                surface_label: row.surface_label.clone(),
                qualification: row.qualification,
                status: row.derived_status,
                lifecycle_label: row.lifecycle_label,
                certified_doc_field_count: row.certified_doc_fields.len(),
                certified_parity_card_count: row.certified_parity_cards.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                documentation_record: row.documentation_record,
                cross_surface_naming: row.cross_surface_naming,
                example_freshness: row.example_freshness,
                doc_export_parity: row.doc_export_parity,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .conformance_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_COMMAND_DOC_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_COMMAND_DOC_SCHEMA_VERSION,
            dashboard_id: M5_COMMAND_DOC_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            command_automation_refs: packet.command_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 command-documentation dashboard serializes")
    }
}

/// Support-export wrapper for the documentation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDocSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: CommandDocPacket,
    /// Dashboard quoted in full.
    pub dashboard: CommandDocDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl CommandDocSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each surface family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the migration tooling — can name the
    /// same surface and waiver the runtime certified.
    pub fn from_packet(support_export_id: impl Into<String>, packet: CommandDocPacket) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.surface_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_COMMAND_DOC_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_COMMAND_DOC_SCHEMA_VERSION,
            shared_contract_ref: M5_COMMAND_DOC_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_command_documentation_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandDocInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen discoverability matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family documentation rows.
    pub rows: Vec<CommandDocRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The documentation packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`CommandDocPacket`] from the exact build identity, the frozen matrix ref, and the per-family
/// documentation rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the auto-narrowing
/// cannot be asserted.
pub fn build_m5_command_documentation_packet(input: CommandDocInput) -> CommandDocPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<CommandDocRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<CommandDocFinding> = Vec::new();

    // Every surface family must carry a documentation row.
    let present: BTreeSet<M5CommandSurfaceFamily> =
        rows.iter().map(|row| row.surface_family).collect();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(CommandDocFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_surface_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandDocStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandDocStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandDocStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(CommandDocFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<CommandDocWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<CommandDocCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_doc_dimensions: Vec<String> = REQUIRED_DOC_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_doc_fields: Vec<String> = REQUIRED_DOC_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    let required_parity_cards: Vec<String> = REQUIRED_PARITY_CARDS
        .iter()
        .map(|card| card.as_str().to_owned())
        .collect();
    let required_derivation_anchors: Vec<String> = REQUIRED_DERIVATION_ANCHORS
        .iter()
        .map(|anchor| anchor.as_str().to_owned())
        .collect();
    let required_surface_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = CommandDocPacket {
        record_kind: M5_COMMAND_DOC_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_COMMAND_DOC_SCHEMA_VERSION,
        shared_contract_ref: M5_COMMAND_DOC_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_COMMAND_DOC_PACKET_ID.to_owned(),
        source_schema_ref: M5_COMMAND_DOC_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Command documentation for every claimed M5 command surface: each of the ten governed \
                   surface families certified so a reader, doc, automation, or support reviewer sees one \
                   canonical command record — the same command id, primary label, aliases, lifecycle / \
                   deprecation state, supported surfaces, invocation-schema summary, side-effect / risk \
                   class, and result / rollback semantics — with canonical examples that stay fresh and \
                   never alias-only, parity cards that show the same command across menus, buttons, the \
                   palette, CLI/headless, AI tools, recipes, and voice/companion hints, and a copy-safe, \
                   diffable export that reconstructs the command id and replacement guidance, across every \
                   declared consumer surface, with the same documentation preserved in headless/CLI \
                   execution, each surface's green/yellow/red claim auto-narrowed from its four \
                   documentation postures, and any surface that ships a mismatched record, drifts on \
                   naming or replacement guidance, ships a stale or alias-only example, or cannot \
                   reconstruct its command id from durable evidence blocked from a stable claim."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_COMMAND_DOC_MATRIX_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_COMMAND_DOC_MATRIX_DOC_REF.to_owned(),
        command_descriptor_ref: M5_COMMAND_DOC_COMMAND_DESCRIPTOR_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_doc_dimensions,
        required_doc_fields,
        required_parity_cards,
        required_derivation_anchors,
        required_surface_families,
        rows,
        covered_surface_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        conformance_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        command_automation_refs: vec![
            "command_status.documentation_registry".to_owned(),
            "documentation_automation.auto_narrow.command_documentation_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.command_documentation".to_owned(),
            M5_COMMAND_DOC_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_COMMAND_DOC_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-command-documentation".to_owned()],
        published_report_ref: M5_COMMAND_DOC_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_COMMAND_DOC_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_COMMAND_DOC_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_COMMAND_DOC_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("documentation packet serializes"),
    ) {
        blocking_findings.push(CommandDocFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_command_documentation_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CommandDocValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required documentation dimensions do not match the lane constants.
    RequiredDocDimensionsStale,
    /// The declared required documentation fields do not match the lane constants.
    RequiredDocFieldsStale,
    /// The declared required parity cards do not match the lane constants.
    RequiredParityCardsStale,
    /// The declared required derivation anchors do not match the lane constants.
    RequiredDerivationAnchorsStale,
    /// The declared required surface families do not match the lane constants.
    RequiredSurfaceFamiliesStale,
    /// The rows do not cover all ten surface families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared conformance causes do not match the recomputed causes.
    ConformanceCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the documentation invariants.
///
/// The checks encode the track invariant and acceptance criteria: every surface family carries a current
/// documentation row; each row's status is the derived value, never asserted; a green row cannot keep a
/// claim while it ships a mismatched record, drifts on naming or replacement guidance, ships a stale or
/// alias-only example, cannot reconstruct its command id from durable evidence, loses headless/CLI parity,
/// fails to publish all eight documentation-record fields, fails to render all seven parity cards, fails to
/// derive all three derivation anchors, or fails to certify every declared consumer surface; and a
/// disclosed narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_command_documentation_packet(
    packet: &CommandDocPacket,
) -> Result<(), Vec<CommandDocValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(CommandDocValidationError::NoRows);
    }
    if packet.record_kind != M5_COMMAND_DOC_PACKET_RECORD_KIND {
        errors.push(CommandDocValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_COMMAND_DOC_SCHEMA_VERSION {
        errors.push(CommandDocValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(CommandDocValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(CommandDocValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_DOC_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_doc_dimensions != expected_dimensions {
        errors.push(CommandDocValidationError::RequiredDocDimensionsStale);
    }
    let expected_doc_fields: Vec<String> = REQUIRED_DOC_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    if packet.required_doc_fields != expected_doc_fields {
        errors.push(CommandDocValidationError::RequiredDocFieldsStale);
    }
    let expected_parity_cards: Vec<String> = REQUIRED_PARITY_CARDS
        .iter()
        .map(|card| card.as_str().to_owned())
        .collect();
    if packet.required_parity_cards != expected_parity_cards {
        errors.push(CommandDocValidationError::RequiredParityCardsStale);
    }
    let expected_derivation_anchors: Vec<String> = REQUIRED_DERIVATION_ANCHORS
        .iter()
        .map(|anchor| anchor.as_str().to_owned())
        .collect();
    if packet.required_derivation_anchors != expected_derivation_anchors {
        errors.push(CommandDocValidationError::RequiredDerivationAnchorsStale);
    }
    let expected_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_surface_families != expected_families {
        errors.push(CommandDocValidationError::RequiredSurfaceFamiliesStale);
    }

    let present: BTreeSet<M5CommandSurfaceFamily> =
        packet.rows.iter().map(|row| row.surface_family).collect();
    let coverage_complete = REQUIRED_SURFACE_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_SURFACE_FAMILIES.len() {
        errors.push(CommandDocValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_surface_families {
        errors.push(CommandDocValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandDocStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandDocStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandDocStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(CommandDocValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<CommandDocWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(CommandDocValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<CommandDocCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(CommandDocValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<CommandDocFinding> = Vec::new();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(CommandDocFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(CommandDocFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("documentation packet serializes"),
    ) {
        recomputed.push(CommandDocFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(CommandDocValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(CommandDocValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(CommandDocValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(CommandDocValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(CommandDocValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(CommandDocValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
