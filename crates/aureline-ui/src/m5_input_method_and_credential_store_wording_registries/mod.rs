//! Implemented M5 input-method-behavior and credential-store-wording registries.
//!
//! The frozen [platform-fit matrix][matrix] names Aureline's six platform-fit families and locks their
//! controlled vocabulary. This module is the implement lane for the everyday text-entry and secure-store
//! wording flows: it turns the concrete *IME / dead-key / AltGr / dictation / emoji / layout-switch* grammar
//! of the input-method family and the *truthful, non-leaky credential-store wording* grammar of the
//! credential-store-wording family into registry resolvers that produce export-safe, honest projections. A
//! user can then trust that text entered through supported platform input methods arrives intact and correctly
//! segmented across every claimed macOS, Windows, and Linux desktop profile, that shortcut handling and text
//! composition never fight each other, that command interpretation and trust / approval copy on protected
//! paths stay uncorrupted, that credential-store copy stays truthful and privacy-safe by default, and that a
//! surface which corrupts composed text, hijacks a shortcut, or hides a plaintext storage downgrade degrades
//! honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Validate IME composition, dead keys, AltGr, dictation, emoji input, and layout switching across
//!   editor, terminal, settings, dialogs, prompts, and support forms so text arrives intact and correctly
//!   segmented.** [`resolve_input_composition_entry`] refuses to read as a clean, registry-bound input entry
//!   unless it names a canonical registry token, a classified [input-method stack][M5InputMethodStack], an
//!   input-method role, covers every [presentation form][M5InputCredentialPresentationForm] (the literal
//!   committed text, the canonical command / routing truth, and the accessible composition announcement),
//!   delivers committed text that matches the expected text for its stack, preserves command and trust
//!   fidelity, and explains any unsupported-composition fallback; otherwise it degrades.
//! * **Guarantee that platform input methods do not corrupt text fidelity, command interpretation, shortcut
//!   routing, or trust / approval copy on protected paths.** [`input_composition_matches_stack`] rejects an
//!   entry whose committed text drifts from the expected text so a corrupted composition degrades to
//!   [`M5InputCompositionEntryDegradeReason::ComposedTextCorruptedForStack`], and the
//!   `preserves_command_and_trust_fidelity` invariant degrades a composition that fights shortcut routing or
//!   rewrites trust copy to
//!   [`M5InputCompositionEntryDegradeReason::CommandOrTrustFidelityNotPreserved`].
//! * **Use truthful but generic credential-store wording by default, surfacing platform-specific detail only
//!   when it materially helps recovery, repair, or admin diagnosis, and never hiding a plaintext downgrade.**
//!   [`resolve_credential_store_wording_entry`] names a classified [credential-wording
//!   surface][M5CredentialWordingSurface], requires the copy to provide the generic-wording / disclosure-route
//!   / truthful-and-non-leaky disclosure triple, covers every presentation form, and degrades to
//!   [`M5CredentialStoreWordingEntryDegradeReason::StorageClaimUntruthfulOrLeaky`] when the wording names the
//!   disallowed hidden-plaintext-fallback role, leaks a secret, hides a storage downgrade, or asserts false
//!   certainty, so credential copy can never read as truthful when it is not.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5PlatformFitRole`] role vocabulary, the
//! [`M5InputMethodRole`] input-method-role vocabulary, and the [`M5CredentialStoreWordingRole`]
//! credential-store-wording-role vocabulary — so editor, terminal, settings, auth, docs, CLI, and support
//! surfaces can never fork their own text-entry or credential-wording meaning. Raw secret values and private
//! endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_platform_fit_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_input_method_and_credential_store_wording_registries,
    seeded_m5_input_method_and_credential_store_wording_registries_composition_beta_narrowed,
    seeded_m5_input_method_and_credential_store_wording_registries_credential_preview_narrowed,
    M5_INPUT_CREDENTIAL_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_platform_fit_matrix::{
    M5CredentialStoreWordingRole, M5InputMethodRole, M5PlatformFitAccessibilityRoute,
    M5PlatformFitConsumerSurface, M5PlatformFitDeploymentLine, M5PlatformFitDowngradeTrigger,
    M5PlatformFitFamily, M5PlatformFitQualificationClass, M5PlatformFitRequiredLabel,
    M5PlatformFitRole, M5_FILE_PATH_AND_REVEAL_SCHEMA_REF, M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
    M5_PLATFORM_FIT_MATRIX_DOC_REF, M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5InputCredentialRegistriesPacket`].
pub const M5_INPUT_CREDENTIAL_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_input_method_and_credential_store_wording_registries";

/// Schema version for M5 input-method / credential-store-wording registry records.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_REF: &str =
    "schemas/platform/m5-input-method-and-credential-store-wording-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_DOC_REF: &str =
    "docs/platform/m5_input_method_and_credential_store_wording_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-input-method-and-credential-store-wording-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-input-method-and-credential-store-wording-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-input-method-and-credential-store-wording-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/platform/m5-input-method-and-credential-store-wording-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5InputCredentialRegistriesConsumerSurface = M5PlatformFitConsumerSurface;

/// One of the three presentation forms every input-composition or credential-wording entry must hold across so
/// text or wording keeps its truth whether it is shown in its literal committed / rendered form, resolved to
/// its canonical command / storage truth, or announced to a screen reader. Minted by this lane because the
/// frozen matrix names the input-method and credential-store-wording *families* but not the concrete form set
/// an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputCredentialPresentationForm {
    /// The literal committed text (input entry) or the literal rendered wording (credential entry).
    LiteralRendering,
    /// The canonical command / routing truth (input entry) or the truthful storage claim (credential entry),
    /// kept explicit alongside the literal rendering.
    CanonicalTruth,
    /// The spoken / searchable accessible announcement that keeps the composition or wording discoverable.
    AccessibleAnnouncement,
}

impl M5InputCredentialPresentationForm {
    /// Every presentation form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::LiteralRendering,
        Self::CanonicalTruth,
        Self::AccessibleAnnouncement,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiteralRendering => "literal_rendering",
            Self::CanonicalTruth => "canonical_truth",
            Self::AccessibleAnnouncement => "accessible_announcement",
        }
    }
}

/// Controlled input-method stack a composition entry adapts to, so the canonical composition model shares one
/// registry rather than a hand-copied per-platform assumption. Minted by this lane because the frozen matrix
/// carries the macOS / Windows / Linux surface families but not the concrete IME / composition model an input
/// entry validates against. Every classified stack carries its canonical composition model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputMethodStack {
    /// The macOS input-method stack (marked-text composition).
    MacosInputMethods,
    /// The Windows input-method stack (Text Services Framework / IME composition).
    WindowsImeTsf,
    /// The Linux input-method stack (IBus / fcitx preedit composition).
    LinuxImeIbusFcitx,
    /// The input-method stack is unclassified, which is disallowed.
    StackUnclassified,
}

impl M5InputMethodStack {
    /// Every input-method stack, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MacosInputMethods,
        Self::WindowsImeTsf,
        Self::LinuxImeIbusFcitx,
        Self::StackUnclassified,
    ];

    /// The three canonical desktop stacks every claimed M5 profile validates input fidelity against.
    pub const CANONICAL_STACKS: [Self; 3] = [
        Self::MacosInputMethods,
        Self::WindowsImeTsf,
        Self::LinuxImeIbusFcitx,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacosInputMethods => "macos_input_methods",
            Self::WindowsImeTsf => "windows_ime_tsf",
            Self::LinuxImeIbusFcitx => "linux_ime_ibus_fcitx",
            Self::StackUnclassified => "stack_unclassified",
        }
    }

    /// Whether the stack is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::StackUnclassified)
    }

    /// The canonical composition model for this stack.
    pub const fn canonical_composition_model(self) -> &'static str {
        match self {
            Self::MacosInputMethods => "marked-text composition",
            Self::WindowsImeTsf => "tsf composition",
            Self::LinuxImeIbusFcitx => "preedit composition",
            Self::StackUnclassified => "",
        }
    }
}

/// Controlled credential-wording surface a credential entry must render truthful copy from, so a
/// credential-store message shares one registry rather than a hand-copied per-surface string. Minted by this
/// lane, tracking the product surfaces the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialWordingSurface {
    /// The settings credential panel.
    SettingsCredentialPanel,
    /// The auth error / recovery dialog.
    AuthErrorDialog,
    /// The support diagnostics surface.
    SupportDiagnostics,
    /// The credential-wording surface is unclassified, which is disallowed.
    SurfaceUnclassified,
}

impl M5CredentialWordingSurface {
    /// Every credential-wording surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SettingsCredentialPanel,
        Self::AuthErrorDialog,
        Self::SupportDiagnostics,
        Self::SurfaceUnclassified,
    ];

    /// The three canonical surfaces every credential-store message must stay truthful across.
    pub const CANONICAL_SURFACES: [Self; 3] = [
        Self::SettingsCredentialPanel,
        Self::AuthErrorDialog,
        Self::SupportDiagnostics,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsCredentialPanel => "settings_credential_panel",
            Self::AuthErrorDialog => "auth_error_dialog",
            Self::SupportDiagnostics => "support_diagnostics",
            Self::SurfaceUnclassified => "surface_unclassified",
        }
    }

    /// Whether the credential-wording surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SurfaceUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so an input or credential
/// token's meaning stays stable whether it appears in the editor, terminal, a settings field, a modal dialog,
/// or a prompt / support form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputSurfaceContext {
    /// The editor buffer surface.
    EditorBuffer,
    /// The terminal input surface.
    TerminalInput,
    /// The settings field surface.
    SettingsField,
    /// The modal dialog surface.
    ModalDialog,
    /// The prompt / support-form surface.
    PromptOrSupportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5InputSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EditorBuffer,
        Self::TerminalInput,
        Self::SettingsField,
        Self::ModalDialog,
        Self::PromptOrSupportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::EditorBuffer,
        Self::TerminalInput,
        Self::SettingsField,
        Self::ModalDialog,
        Self::PromptOrSupportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorBuffer => "editor_buffer",
            Self::TerminalInput => "terminal_input",
            Self::SettingsField => "settings_field",
            Self::ModalDialog => "modal_dialog",
            Self::PromptOrSupportForm => "prompt_or_support_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part an input-composition or credential-wording entry must be able to show, so no
/// committed text, composition model, credential wording, or registry fact is left implicit behind a
/// hand-copied per-platform assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputCredentialAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The input-method stack the entry adapts to (input entry).
    InputStack,
    /// The committed text the entry delivers (input entry).
    CommittedText,
    /// The presentation-form coverage (literal / canonical / accessible).
    PresentationFormCoverage,
    /// The composition model the entry validates against (input entry).
    CompositionModel,
    /// The credential wording the entry maps (credential entry).
    CredentialWording,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the composition or wording (both entries).
    PlainLanguageMeaning,
}

impl M5InputCredentialAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::InputStack,
        Self::CommittedText,
        Self::PresentationFormCoverage,
        Self::CompositionModel,
        Self::CredentialWording,
        Self::SurfaceContext,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::InputStack => "input_stack",
            Self::CommittedText => "committed_text",
            Self::PresentationFormCoverage => "presentation_form_coverage",
            Self::CompositionModel => "composition_model",
            Self::CredentialWording => "credential_wording",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect composed
/// text, a composition model, credential wording, or a degraded input / credential entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputCredentialNextAction {
    /// Expand the composition's or wording's plain-language meaning.
    ExpandInputMeaning,
    /// Inspect the input stack or credential surface the entry maps.
    InspectStackOrSurface,
    /// Complete the literal / canonical / accessible presentation-form coverage.
    CompletePresentationFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5InputCredentialNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandInputMeaning,
        Self::InspectStackOrSurface,
        Self::CompletePresentationFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInputMeaning => "expand_input_meaning",
            Self::InspectStackOrSurface => "inspect_stack_or_surface",
            Self::CompletePresentationFormCoverage => "complete_presentation_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputCredentialExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The platform-fit families covered.
    PlatformFitFamilies,
    /// The input-method stacks carried.
    InputStacks,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The presentation forms covered.
    PresentationForms,
    /// The credential-wording surfaces carried.
    CredentialWordingSurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The composition models carried.
    CompositionModels,
    /// The accountable owner role.
    OwnerRole,
}

impl M5InputCredentialExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::PlatformFitFamilies,
        Self::InputStacks,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::PresentationForms,
        Self::CredentialWordingSurfaces,
        Self::SurfaceContext,
        Self::CompositionModels,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::PlatformFitFamilies,
        Self::InputStacks,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::PlatformFitFamilies => "platform_fit_families",
            Self::InputStacks => "input_stacks",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::PresentationForms => "presentation_forms",
            Self::CredentialWordingSurfaces => "credential_wording_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::CompositionModels => "composition_models",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an input-composition entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, text-corrupting, fidelity-losing, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputCompositionEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the composition means.
    InputTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The input-method stack is unclassified (not in the preserved taxonomy).
    InputStackUnclassified,
    /// The behavior is a hand-copied per-platform assumption instead of tracing to the canonical registry, or
    /// names the disallowed text-or-trust-corruption role.
    BehaviorNotBoundToRegistry,
    /// The committed text drifts from the expected text for the stack: text did not arrive intact / correctly
    /// segmented.
    ComposedTextCorruptedForStack,
    /// The entry does not preserve command interpretation, shortcut routing, or trust / approval copy on a
    /// protected path (composition and shortcuts fight, or a composition rewrites trust copy).
    CommandOrTrustFidelityNotPreserved,
    /// The literal / canonical / accessible presentation-form coverage is incomplete.
    FidelityFormCoverageIncomplete,
    /// Composition is unsupported on this surface and no fallback input path is explained.
    CompositionUnsupportedWithoutFallback,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5InputCompositionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::InputTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::InputStackUnclassified,
        Self::BehaviorNotBoundToRegistry,
        Self::ComposedTextCorruptedForStack,
        Self::CommandOrTrustFidelityNotPreserved,
        Self::FidelityFormCoverageIncomplete,
        Self::CompositionUnsupportedWithoutFallback,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InputTokenUnstated => "input_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::InputStackUnclassified => "input_stack_unclassified",
            Self::BehaviorNotBoundToRegistry => "behavior_not_bound_to_registry",
            Self::ComposedTextCorruptedForStack => "composed_text_corrupted_for_stack",
            Self::CommandOrTrustFidelityNotPreserved => "command_or_trust_fidelity_not_preserved",
            Self::FidelityFormCoverageIncomplete => "fidelity_form_coverage_incomplete",
            Self::CompositionUnsupportedWithoutFallback => {
                "composition_unsupported_without_fallback"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5InputCredentialNextAction {
        match self {
            Self::InputTokenUnstated | Self::BehaviorNotBoundToRegistry => {
                M5InputCredentialNextAction::TraceCanonicalRegistry
            }
            Self::InputStackUnclassified
            | Self::ComposedTextCorruptedForStack
            | Self::CommandOrTrustFidelityNotPreserved => {
                M5InputCredentialNextAction::InspectStackOrSurface
            }
            Self::FidelityFormCoverageIncomplete => {
                M5InputCredentialNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::CompositionUnsupportedWithoutFallback
            | Self::ProofStale => M5InputCredentialNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::InputTokenUnstated | Self::FidelityFormCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::InputStackUnclassified => M5PlatformFitDowngradeTrigger::HostPlatformUnstated,
            Self::BehaviorNotBoundToRegistry
            | Self::ComposedTextCorruptedForStack
            | Self::CompositionUnsupportedWithoutFallback => {
                M5PlatformFitDowngradeTrigger::InputMethodCorruptedTextOrTrust
            }
            Self::CommandOrTrustFidelityNotPreserved => {
                M5PlatformFitDowngradeTrigger::PlatformWordingChangedCommandOrPermissionMeaning
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a credential-store-wording entry degraded below a clean, truthful state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialStoreWordingEntryDegradeReason {
    /// The canonical registry token name is unstated.
    WordingTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The credential-wording surface is unclassified (not in the preserved taxonomy).
    CredentialSurfaceUnclassified,
    /// The wording is untruthful or leaky — it names the disallowed hidden-plaintext-fallback role, leaks a
    /// secret, hides a storage downgrade, asserts false certainty, or drops the generic-wording /
    /// disclosure-route / truthful-and-non-leaky disclosure triple.
    StorageClaimUntruthfulOrLeaky,
    /// The literal / canonical / accessible presentation-form coverage of the credential wording is
    /// incomplete.
    WordingPhrasingCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CredentialStoreWordingEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WordingTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CredentialSurfaceUnclassified,
        Self::StorageClaimUntruthfulOrLeaky,
        Self::WordingPhrasingCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WordingTokenUnstated => "wording_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CredentialSurfaceUnclassified => "credential_surface_unclassified",
            Self::StorageClaimUntruthfulOrLeaky => "storage_claim_untruthful_or_leaky",
            Self::WordingPhrasingCoverageIncomplete => "wording_phrasing_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5InputCredentialNextAction {
        match self {
            Self::WordingTokenUnstated => M5InputCredentialNextAction::TraceCanonicalRegistry,
            Self::CredentialSurfaceUnclassified | Self::StorageClaimUntruthfulOrLeaky => {
                M5InputCredentialNextAction::InspectStackOrSurface
            }
            Self::WordingPhrasingCoverageIncomplete => {
                M5InputCredentialNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5InputCredentialNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::WordingTokenUnstated => M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::CredentialSurfaceUnclassified => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::StorageClaimUntruthfulOrLeaky => {
                M5PlatformFitDowngradeTrigger::SecretStorageFellBackToPlaintextSilently
            }
            Self::WordingPhrasingCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_input_composition_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InputCompositionEntryResolutionInput {
    /// Stable identity of the input-composition-registry entry.
    pub entry_id: String,
    /// The stable command ID this composition binds to (e.g. `command.editor.insert`); empty means unstated.
    pub command_id: String,
    /// The canonical registry token name (e.g. `input.ime.editor`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The input-method role (from the frozen matrix vocabulary).
    pub input_role: M5InputMethodRole,
    /// The input-method stack this entry adapts to.
    pub input_stack: M5InputMethodStack,
    /// The render / surface context.
    pub surface_context: M5InputSurfaceContext,
    /// The presentation forms this entry holds across (must cover literal / canonical / accessible).
    pub presentation_form_coverage: Vec<M5InputCredentialPresentationForm>,
    /// The committed text that arrives after composition (e.g. the composed CJK / emoji / dead-key result).
    pub committed_text: String,
    /// The expected text for this stack (what a correct composition should deliver).
    pub expected_text: String,
    /// True when the behavior traces to the shared input registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the entry preserves command interpretation, shortcut routing, and trust / approval copy (a
    /// hard invariant when `false`).
    pub preserves_command_and_trust_fidelity: bool,
    /// True when composition is unsupported on this surface (e.g. a raw terminal without an IME bridge).
    pub composition_unsupported_on_surface: bool,
    /// True when an explicit fallback input path is explained for an unsupported-composition surface.
    pub fallback_input_path_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe input-composition-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInputCompositionEntry {
    /// Stable identity of the input-composition-registry entry.
    pub entry_id: String,
    /// The stable command ID this composition binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve command identity as platform labels and notation adapt.
    pub semantic_role_preserves_command_identity_under_platform_adaptation: bool,
    /// The input-method-role token named by the entry.
    pub input_role: String,
    /// Whether the input role names the disallowed text-or-trust-corruption token.
    pub input_role_names_corruption: bool,
    /// The input-method-stack token named by the entry.
    pub input_stack: String,
    /// Whether the input-method stack is classified into the preserved taxonomy.
    pub input_stack_is_classified: bool,
    /// The canonical composition model for the entry's input stack.
    pub canonical_composition_model: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The committed text delivered by the entry.
    pub committed_text: String,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// Whether the committed text arrived intact and correctly segmented for the stack.
    pub composed_text_intact: bool,
    /// Whether the entry traces to the shared input registry.
    pub bound_to_registry: bool,
    /// Whether the entry preserves command interpretation, shortcut routing, and trust / approval copy.
    pub preserves_command_and_trust_fidelity: bool,
    /// Whether composition is unsupported on this surface.
    pub composition_unsupported_on_surface: bool,
    /// Whether an explicit fallback input path is explained for an unsupported-composition surface.
    pub fallback_input_path_explained: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5InputCompositionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5InputCredentialNextAction,
    /// Whether text fidelity holds across every presentation form and stack (clean entry naming every fact).
    pub text_fidelity_holds_across_surfaces_and_profiles: bool,
}

impl M5ResolvedInputCompositionEntry {
    /// Whether this input-composition entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_credential_store_wording_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CredentialStoreWordingEntryResolutionInput {
    /// Stable identity of the credential-store-wording entry.
    pub entry_id: String,
    /// The stable command ID this wording binds to; empty means unstated.
    pub command_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The credential-store-wording role this entry carries (from the frozen matrix vocabulary).
    pub wording_role: M5CredentialStoreWordingRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The credential-wording surface this entry must stay truthful on.
    pub wording_surface: M5CredentialWordingSurface,
    /// The render / surface context.
    pub surface_context: M5InputSurfaceContext,
    /// The presentation forms this entry holds across (must cover literal / canonical / accessible).
    pub presentation_form_coverage: Vec<M5InputCredentialPresentationForm>,
    /// The truthful, generic credential-store copy shown by default; empty means missing.
    pub generic_wording: String,
    /// The route to recovery / repair / admin detail the wording points at; empty means missing.
    pub disclosure_route: String,
    /// True when the storage claim is truthful (never a false "encrypted" over a plaintext downgrade).
    pub storage_is_truthful: bool,
    /// True when the wording is non-leaky (never surfaces a secret value).
    pub non_leaky: bool,
    /// True when a plaintext fallback was used for this credential.
    pub plaintext_fallback_used: bool,
    /// True when a used plaintext fallback is disclosed rather than hidden.
    pub plaintext_fallback_disclosed: bool,
    /// True when platform-specific detail is disclosed in the wording.
    pub platform_detail_disclosed: bool,
    /// True when disclosed platform-specific detail is justified by material recovery / repair / admin value.
    pub platform_detail_justified: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe credential-store-wording projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCredentialStoreWordingEntry {
    /// Stable identity of the credential-store-wording entry.
    pub entry_id: String,
    /// The stable command ID this wording binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The credential-store-wording-role token named by the entry.
    pub wording_role: String,
    /// Whether the wording role names the disallowed hidden-plaintext-fallback token.
    pub wording_role_hides_plaintext: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The credential-wording-surface token named by the entry.
    pub wording_surface: String,
    /// Whether the credential-wording surface is classified into the preserved taxonomy.
    pub wording_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// The truthful, generic credential-store copy named by the entry.
    pub generic_wording: String,
    /// The recovery / repair / admin disclosure route named by the entry.
    pub disclosure_route: String,
    /// Whether the storage claim is truthful.
    pub storage_is_truthful: bool,
    /// Whether the wording is non-leaky.
    pub non_leaky: bool,
    /// Whether a plaintext fallback was used.
    pub plaintext_fallback_used: bool,
    /// Whether a used plaintext fallback is disclosed.
    pub plaintext_fallback_disclosed: bool,
    /// Whether platform-specific detail is disclosed.
    pub platform_detail_disclosed: bool,
    /// Whether disclosed platform-specific detail is justified.
    pub platform_detail_justified: bool,
    /// Whether the wording stays truthful and privacy-safe (no leak, no hidden downgrade, no unjustified
    /// platform detail).
    pub wording_stays_truthful: bool,
    /// Whether the entry provides the complete generic-wording / disclosure-route / truthful-and-non-leaky
    /// disclosure triple.
    pub provides_complete_disclosure_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, truthful state.
    pub degrade_reason: Option<M5CredentialStoreWordingEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5InputCredentialNextAction,
    /// Whether the wording is truthful on every claimed desktop profile (clean entry naming every fact).
    pub wording_truthful_on_every_profile: bool,
}

impl M5ResolvedCredentialStoreWordingEntry {
    /// Whether this credential-store-wording entry reads as a clean, truthful state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5InputCredentialResolutionError {
    /// The input-composition-entry id was empty.
    EmptyInputCompositionEntryId,
    /// The credential-store-wording-entry id was empty.
    EmptyCredentialStoreWordingEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5InputCredentialResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyInputCompositionEntryId => "empty_input_composition_entry_id",
            Self::EmptyCredentialStoreWordingEntryId => "empty_credential_store_wording_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5InputCredentialResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 input-method / credential-store-wording registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5InputCredentialResolutionError {}

fn form_tokens(forms: &[M5InputCredentialPresentationForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_presentation_forms(forms: &[M5InputCredentialPresentationForm]) -> bool {
    let present: BTreeSet<M5InputCredentialPresentationForm> = forms.iter().copied().collect();
    M5InputCredentialPresentationForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the committed text arrived intact and correctly segmented for the input-method stack: the stack
/// must be classified, the committed text and the expected text must both be present, and the committed text
/// must equal the expected text. An unclassified stack, an empty committed or expected text, or any drift
/// between them never matches.
pub fn input_composition_matches_stack(
    stack: M5InputMethodStack,
    committed_text: &str,
    expected_text: &str,
) -> bool {
    if !stack.is_classified() || committed_text.trim().is_empty() || expected_text.trim().is_empty()
    {
        return false;
    }
    committed_text == expected_text
}

/// Whether credential-store wording stays truthful and privacy-safe: the surface must be classified, the
/// storage claim must be truthful, the wording must be non-leaky, any used plaintext fallback must be
/// disclosed rather than hidden, and any disclosed platform-specific detail must be justified by material
/// recovery / repair / admin value.
#[allow(clippy::too_many_arguments)]
pub fn credential_wording_stays_truthful(
    surface: M5CredentialWordingSurface,
    storage_is_truthful: bool,
    non_leaky: bool,
    plaintext_fallback_used: bool,
    plaintext_fallback_disclosed: bool,
    platform_detail_disclosed: bool,
    platform_detail_justified: bool,
) -> bool {
    surface.is_classified()
        && storage_is_truthful
        && non_leaky
        && (!plaintext_fallback_used || plaintext_fallback_disclosed)
        && (!platform_detail_disclosed || platform_detail_justified)
}

/// Resolves an input-composition-registry entry so it stays bound to the shared input registry: the entry
/// names its canonical token, semantic role, input role, and input stack, covers all three presentation
/// forms, delivers committed text that matches the expected text for its stack, preserves command and trust
/// fidelity, and explains any unsupported-composition fallback.
pub fn resolve_input_composition_entry(
    input: M5InputCompositionEntryResolutionInput,
) -> Result<M5ResolvedInputCompositionEntry, M5InputCredentialResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5InputCredentialResolutionError::EmptyInputCompositionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.committed_text)
        || string_is_forbidden(&input.expected_text)
    {
        return Err(M5InputCredentialResolutionError::ForbiddenMaterial);
    }

    let input_role_names_corruption = matches!(
        input.input_role,
        M5InputMethodRole::TextOrTrustCorruptionDisallowed
    );
    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let text_intact = input_composition_matches_stack(
        input.input_stack,
        &input.committed_text,
        &input.expected_text,
    );
    let composition_unhandled =
        input.composition_unsupported_on_surface && !input.fallback_input_path_explained;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5InputCompositionEntryDegradeReason::InputTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5InputCompositionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.input_stack.is_classified() {
        Some(M5InputCompositionEntryDegradeReason::InputStackUnclassified)
    } else if input_role_names_corruption || !input.bound_to_registry {
        Some(M5InputCompositionEntryDegradeReason::BehaviorNotBoundToRegistry)
    } else if !text_intact {
        Some(M5InputCompositionEntryDegradeReason::ComposedTextCorruptedForStack)
    } else if !input.preserves_command_and_trust_fidelity {
        Some(M5InputCompositionEntryDegradeReason::CommandOrTrustFidelityNotPreserved)
    } else if !all_forms {
        Some(M5InputCompositionEntryDegradeReason::FidelityFormCoverageIncomplete)
    } else if composition_unhandled {
        Some(M5InputCompositionEntryDegradeReason::CompositionUnsupportedWithoutFallback)
    } else if !input.proof_fresh {
        Some(M5InputCompositionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5InputCredentialNextAction::ExpandInputMeaning,
    };

    Ok(M5ResolvedInputCompositionEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_command_identity_under_platform_adaptation: input
            .semantic_role
            .must_preserve_command_identity_under_platform_adaptation(),
        input_role: input.input_role.as_str().to_owned(),
        input_role_names_corruption,
        input_stack: input.input_stack.as_str().to_owned(),
        input_stack_is_classified: input.input_stack.is_classified(),
        canonical_composition_model: input.input_stack.canonical_composition_model().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        committed_text: input.committed_text,
        presentation_form_coverage: form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        composed_text_intact: text_intact,
        bound_to_registry: input.bound_to_registry,
        preserves_command_and_trust_fidelity: input.preserves_command_and_trust_fidelity,
        composition_unsupported_on_surface: input.composition_unsupported_on_surface,
        fallback_input_path_explained: input.fallback_input_path_explained,
        degrade_reason,
        next_action,
        text_fidelity_holds_across_surfaces_and_profiles: degrade_reason.is_none(),
    })
}

/// Resolves a credential-store-wording entry so credential copy stays truthful and privacy-safe: the entry
/// names its canonical token, wording role, semantic role, and credential surface, covers all three
/// presentation forms, provides the generic-wording / disclosure-route / truthful-and-non-leaky disclosure
/// triple, and degrades honestly when the wording leaks a secret, hides a plaintext downgrade, or asserts
/// false certainty.
pub fn resolve_credential_store_wording_entry(
    input: M5CredentialStoreWordingEntryResolutionInput,
) -> Result<M5ResolvedCredentialStoreWordingEntry, M5InputCredentialResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5InputCredentialResolutionError::EmptyCredentialStoreWordingEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.generic_wording)
        || string_is_forbidden(&input.disclosure_route)
    {
        return Err(M5InputCredentialResolutionError::ForbiddenMaterial);
    }

    let wording_role_hides_plaintext = matches!(
        input.wording_role,
        M5CredentialStoreWordingRole::PlaintextFallbackHiddenDisallowed
    );
    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let wording_stays_truthful = credential_wording_stays_truthful(
        input.wording_surface,
        input.storage_is_truthful,
        input.non_leaky,
        input.plaintext_fallback_used,
        input.plaintext_fallback_disclosed,
        input.platform_detail_disclosed,
        input.platform_detail_justified,
    );
    let provides_triple = input.wording_surface.is_classified()
        && !input.command_id.trim().is_empty()
        && !input.generic_wording.trim().is_empty()
        && !input.disclosure_route.trim().is_empty()
        && wording_stays_truthful;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CredentialStoreWordingEntryDegradeReason::WordingTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CredentialStoreWordingEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.wording_surface.is_classified() {
        Some(M5CredentialStoreWordingEntryDegradeReason::CredentialSurfaceUnclassified)
    } else if wording_role_hides_plaintext || !provides_triple {
        Some(M5CredentialStoreWordingEntryDegradeReason::StorageClaimUntruthfulOrLeaky)
    } else if !all_forms {
        Some(M5CredentialStoreWordingEntryDegradeReason::WordingPhrasingCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5CredentialStoreWordingEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5InputCredentialNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCredentialStoreWordingEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        wording_role: input.wording_role.as_str().to_owned(),
        wording_role_hides_plaintext,
        semantic_role: input.semantic_role.as_str().to_owned(),
        wording_surface: input.wording_surface.as_str().to_owned(),
        wording_surface_is_classified: input.wording_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        presentation_form_coverage: form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        generic_wording: input.generic_wording,
        disclosure_route: input.disclosure_route,
        storage_is_truthful: input.storage_is_truthful,
        non_leaky: input.non_leaky,
        plaintext_fallback_used: input.plaintext_fallback_used,
        plaintext_fallback_disclosed: input.plaintext_fallback_disclosed,
        platform_detail_disclosed: input.platform_detail_disclosed,
        platform_detail_justified: input.platform_detail_justified,
        wording_stays_truthful,
        provides_complete_disclosure_triple: provides_triple,
        degrade_reason,
        next_action,
        wording_truthful_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved input-composition and credential-store-wording
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputCredentialRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5InputCredentialRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5PlatformFitQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5PlatformFitDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5PlatformFitRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5PlatformFitAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5InputCredentialAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5InputCredentialExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    /// Resolved input-composition-registry examples.
    pub input_composition_entries: Vec<M5ResolvedInputCompositionEntry>,
    /// Resolved credential-store-wording examples.
    pub credential_store_wording_entries: Vec<M5ResolvedCredentialStoreWordingEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the input-method-behavior and
    /// file-path-and-reveal domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a platform input method never corrupts text, command interpretation, or trust
    /// fidelity. MUST be `false`.
    pub input_method_corrupts_text_command_or_trust: bool,
    /// Hard invariant: shortcut routing and text composition never fight each other. MUST be `false`.
    pub shortcut_routing_and_composition_fight: bool,
    /// Hard invariant: credential wording never hides a plaintext storage downgrade or leaks a secret. MUST be
    /// `false`.
    pub credential_wording_hides_plaintext_downgrade_or_leaks: bool,
    /// Hard invariant: input or credential wording is never hand-copied per platform instead of tracing to the
    /// registry. MUST be `false`.
    pub input_or_credential_wording_hardcoded_instead_of_registry: bool,
}

impl M5InputCredentialRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5InputCredentialAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5InputCredentialAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5InputCredentialExportField> =
            self.export_fields.iter().copied().collect();
        M5InputCredentialExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.input_method_corrupts_text_command_or_trust
            && !self.shortcut_routing_and_composition_fight
            && !self.credential_wording_hides_plaintext_downgrade_or_leaks
            && !self.input_or_credential_wording_hardcoded_instead_of_registry
    }

    /// True when a clean input entry preserves registry-bound fidelity: it traces to the registry, never names
    /// the disallowed corruption role, keeps a classified input stack, delivers intact committed text,
    /// preserves command and trust fidelity, covers all three presentation forms, and explains any
    /// unsupported-composition fallback.
    fn input_is_honest(ex: &M5ResolvedInputCompositionEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && !ex.input_role_names_corruption
                && ex.input_stack_is_classified
                && ex.composed_text_intact
                && ex.preserves_command_and_trust_fidelity
                && ex.covers_all_presentation_forms
                && (!ex.composition_unsupported_on_surface || ex.fallback_input_path_explained))
    }

    /// True when a clean credential entry preserves truthful wording: it keeps a classified surface, never
    /// names the disallowed hidden-plaintext-fallback role, provides the disclosure triple, and covers all
    /// three presentation forms.
    fn credential_is_honest(ex: &M5ResolvedCredentialStoreWordingEntry) -> bool {
        !ex.is_clean()
            || (ex.wording_surface_is_classified
                && !ex.wording_role_hides_plaintext
                && ex.provides_complete_disclosure_triple
                && ex.covers_all_presentation_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.input_composition_entries
            .iter()
            .all(Self::input_is_honest)
            && self
                .credential_store_wording_entries
                .iter()
                .all(Self::credential_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputCredentialRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Input-method-role tokens (bound from the frozen matrix).
    pub input_roles: Vec<String>,
    /// Credential-store-wording-role tokens (bound from the frozen matrix).
    pub credential_wording_roles: Vec<String>,
    /// Presentation-form tokens (minted by this lane).
    pub presentation_forms: Vec<String>,
    /// Input-method-stack tokens (minted by this lane).
    pub input_stacks: Vec<String>,
    /// Credential-wording-surface tokens (minted by this lane).
    pub credential_wording_surfaces: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Input-composition-entry degrade-reason tokens.
    pub input_composition_degrade_reasons: Vec<String>,
    /// Credential-store-wording-entry degrade-reason tokens.
    pub credential_store_wording_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5InputCredentialRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5PlatformFitRole::ALL, |v| v.as_str()),
            input_roles: tokens(&M5InputMethodRole::ALL, |v| v.as_str()),
            credential_wording_roles: tokens(&M5CredentialStoreWordingRole::ALL, |v| v.as_str()),
            presentation_forms: tokens(&M5InputCredentialPresentationForm::ALL, |v| v.as_str()),
            input_stacks: tokens(&M5InputMethodStack::ALL, |v| v.as_str()),
            credential_wording_surfaces: tokens(&M5CredentialWordingSurface::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5InputSurfaceContext::ALL, |v| v.as_str()),
            input_composition_degrade_reasons: tokens(
                &M5InputCompositionEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            credential_store_wording_degrade_reasons: tokens(
                &M5CredentialStoreWordingEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5InputCredentialAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5InputCredentialNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5InputCredentialExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5PlatformFitConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputCredentialRegistriesGovernanceReview {
    /// The input registry names a canonical token, input role, and input stack for every entry.
    pub input_registry_names_token_role_and_stack: bool,
    /// Text entered through supported input methods arrives intact and correctly segmented from the shared
    /// registry, not per-surface strings.
    pub text_arrives_intact_from_shared_registry: bool,
    /// Command interpretation, shortcut routing, and trust / approval copy stay uncorrupted on protected
    /// paths.
    pub command_shortcut_and_trust_fidelity_preserved: bool,
    /// Shortcut handling and text composition never fight each other under IME / dead-key / AltGr / dictation.
    pub shortcut_handling_and_composition_do_not_fight: bool,
    /// Credential-store copy stays truthful and generic by default, surfacing platform detail only when it
    /// materially helps.
    pub credential_copy_truthful_and_generic_by_default: bool,
    /// Credential wording never hides a plaintext storage downgrade or leaks a secret value.
    pub credential_wording_never_hides_downgrade_or_leaks: bool,
    /// Every input and credential entry covers the literal / canonical / accessible presentation forms.
    pub every_entry_covers_all_presentation_forms: bool,
    /// Input and credential wording stay bound to the shared registries rather than hand-copied per platform.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Docs, help, and screenshots are generated from the same input / credential registries.
    pub docs_help_and_screenshots_generated_from_registry: bool,
    /// Corrupted composition, a shortcut-composition fight, or a hidden storage downgrade is caught by
    /// fixtures before release evidence turns green.
    pub input_or_wording_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputCredentialRegistriesConsumerProjection {
    /// The editor and terminal consume the shared input registry.
    pub editor_and_terminal_consume_shared_registries: bool,
    /// The settings and dialogs consume the shared registries.
    pub settings_and_dialogs_consume_shared_registries: bool,
    /// The auth surface consumes the shared credential-wording registry.
    pub auth_consumes_shared_registries: bool,
    /// Docs, help, onboarding, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical input-method-behavior and credential domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical input / credential registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputCredentialRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputCredentialRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting platform-fit audit for the lane.
    pub platform_fit_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5InputCredentialRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InputCredentialRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5InputCredentialRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InputCredentialRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InputCredentialRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InputCredentialRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InputCredentialRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InputCredentialRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 input-method and credential-store-wording registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputCredentialRegistriesPacket {
    /// Record kind; must equal [`M5_INPUT_CREDENTIAL_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5InputCredentialRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InputCredentialRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InputCredentialRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InputCredentialRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InputCredentialRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InputCredentialRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5InputCredentialRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5InputCredentialRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_INPUT_CREDENTIAL_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5InputCredentialRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_INPUT_CREDENTIAL_REGISTRIES_RECORD_KIND {
            violations.push(M5InputCredentialRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5InputCredentialRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InputCredentialRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5InputCredentialRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 input-method / credential-store-wording registries packet serializes"),
        ) {
            violations.push(M5InputCredentialRegistriesViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 input-method / credential-store-wording registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,input_composition_entries,credential_store_wording_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .input_composition_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.credential_store_wording_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.input_composition_entries.len(),
                row.credential_store_wording_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Input-Method and Credential-Store-Wording Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Input stacks: {}\n",
            self.vocabulary_set.input_stacks.join(", ")
        ));
        out.push_str(&format!(
            "- Presentation forms: {}\n",
            self.vocabulary_set.presentation_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Input entries: {} / credential entries: {}\n",
                row.input_composition_entries.len(),
                row.credential_store_wording_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-stack help / screenshot input-composition table generated from the registry, so docs
    /// and tutorials render the same command / stack / committed-text / composition-model truth the resolvers
    /// produced rather than a hand-copied screenshot. Only clean, registry-bound input entries are listed.
    pub fn render_input_composition_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| command_id | input_stack | committed_text | composition_model | surface |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.input_composition_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | {} | {} |\n",
                    ex.command_id,
                    ex.input_stack,
                    ex.committed_text,
                    ex.canonical_composition_model,
                    ex.surface_context
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5InputCredentialRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InputCredentialRegistriesViolation>),
}

impl fmt::Display for M5InputCredentialRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 input-method / credential-store-wording registries export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 input-method / credential-store-wording registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InputCredentialRegistriesArtifactError {}

/// Validation failures emitted by [`M5InputCredentialRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InputCredentialRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the input-method-behavior and file-path-and-reveal domain
    /// schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, text-corrupting, fidelity-losing,
    /// form-incomplete, or a credential entry missing the disclosure triple).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Text-intact-across-profiles is not proven: clean input entries do not cover the input-fidelity /
    /// command-stability semantic-role families or the first editor / terminal / settings / dialog / prompt
    /// surfaces, no text-corrupted example degrades, or a clean input entry delivered corrupted text.
    TextIntactAcrossProfilesNotProven,
    /// Composition-and-shortcuts-do-not-fight is not proven: no command-or-trust-fidelity example and no
    /// behavior-not-bound example degrade, no clean bound input entry is present, or a clean input entry lost
    /// command / trust fidelity or is unbound.
    CompositionAndShortcutsDoNotFightNotProven,
    /// Credential-copy-truthful-and-privacy-safe is not proven: clean credential entries do not cover the
    /// settings / auth / support surfaces with full presentation-form coverage while providing the disclosure
    /// triple, no untruthful-or-leaky or phrasing-incomplete example degrades, or a clean credential entry is
    /// missing the triple.
    CredentialCopyTruthfulAndPrivacySafeNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5InputCredentialRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::TextIntactAcrossProfilesNotProven => "text_intact_across_profiles_not_proven",
            Self::CompositionAndShortcutsDoNotFightNotProven => {
                "composition_and_shortcuts_do_not_fight_not_proven"
            }
            Self::CredentialCopyTruthfulAndPrivacySafeNotProven => {
                "credential_copy_truthful_and_privacy_safe_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_input_method_and_credential_store_wording_registries_export(
) -> Result<M5InputCredentialRegistriesPacket, M5InputCredentialRegistriesArtifactError> {
    let packet: M5InputCredentialRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-input-method-and-credential-store-wording-registries-proof/support_export.json"
    )))
    .map_err(M5InputCredentialRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InputCredentialRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5InputCredentialRegistriesPacket,
    violations: &mut Vec<M5InputCredentialRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_REF,
        M5_INPUT_CREDENTIAL_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5InputCredentialRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5InputCredentialRegistriesPacket,
    violations: &mut Vec<M5InputCredentialRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5InputCredentialRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5InputCredentialRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5InputCredentialRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5InputCredentialRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF)
            || !refs.contains(M5_FILE_PATH_AND_REVEAL_SCHEMA_REF)
        {
            violations.push(M5InputCredentialRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.input_composition_entries.is_empty()
            || row.credential_store_wording_entries.is_empty()
        {
            violations.push(M5InputCredentialRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5InputCredentialRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5InputCredentialRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5InputCredentialRegistriesPacket,
    violations: &mut Vec<M5InputCredentialRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.input_registry_names_token_role_and_stack,
        review.text_arrives_intact_from_shared_registry,
        review.command_shortcut_and_trust_fidelity_preserved,
        review.shortcut_handling_and_composition_do_not_fight,
        review.credential_copy_truthful_and_generic_by_default,
        review.credential_wording_never_hides_downgrade_or_leaks,
        review.every_entry_covers_all_presentation_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.docs_help_and_screenshots_generated_from_registry,
        review.input_or_wording_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5InputCredentialRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5InputCredentialRegistriesPacket,
    violations: &mut Vec<M5InputCredentialRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_and_terminal_consume_shared_registries,
        projection.settings_and_dialogs_consume_shared_registries,
        projection.auth_consumes_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5InputCredentialRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5InputCredentialRegistriesPacket,
    violations: &mut Vec<M5InputCredentialRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5InputCredentialRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5InputCredentialRegistriesPacket,
    violations: &mut Vec<M5InputCredentialRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.platform_fit_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5InputCredentialRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5InputCredentialRegistriesPacket,
    violations: &mut Vec<M5InputCredentialRegistriesViolation>,
) {
    let inputs = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.input_composition_entries.iter())
    };
    let credentials = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.credential_store_wording_entries.iter())
    };

    // AC1: text entered through supported platform input methods arrives intact and correctly segmented across
    // all claimed desktop profiles. Clean input entries cover the input-fidelity / command-stability
    // semantic-role families and the first editor / terminal / settings / dialog / prompt surfaces, a
    // text-corrupted example degrades, and no clean input entry delivered corrupted text.
    let clean_semantic_roles: BTreeSet<String> = inputs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = inputs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = [
        M5PlatformFitRole::InputFidelity.as_str(),
        M5PlatformFitRole::CommandStability.as_str(),
    ]
    .iter()
    .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5InputSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let text_corrupted_degrades = inputs().any(|ex| {
        ex.degrade_reason
            == Some(M5InputCompositionEntryDegradeReason::ComposedTextCorruptedForStack)
    });
    let no_clean_corrupted = !inputs().any(|ex| ex.is_clean() && !ex.composed_text_intact);
    if !(semantic_families_covered
        && first_surfaces_covered
        && text_corrupted_degrades
        && no_clean_corrupted)
    {
        violations.push(M5InputCredentialRegistriesViolation::TextIntactAcrossProfilesNotProven);
    }

    // AC2: shortcut handling and text composition do not fight each other under IME / dead-key / AltGr /
    // dictation. A command-or-trust-fidelity example degrades, a behavior-not-bound example degrades, at least
    // one clean bound input entry is present, and no clean input entry lost command / trust fidelity or is
    // unbound.
    let fidelity_not_preserved_degrades = inputs().any(|ex| {
        ex.degrade_reason
            == Some(M5InputCompositionEntryDegradeReason::CommandOrTrustFidelityNotPreserved)
    });
    let behavior_not_bound_degrades = inputs().any(|ex| {
        ex.degrade_reason == Some(M5InputCompositionEntryDegradeReason::BehaviorNotBoundToRegistry)
    });
    let bound_clean_input = inputs().any(|ex| ex.is_clean() && ex.bound_to_registry);
    let no_clean_unbound = !inputs().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_lost_fidelity =
        !inputs().any(|ex| ex.is_clean() && !ex.preserves_command_and_trust_fidelity);
    if !(fidelity_not_preserved_degrades
        && behavior_not_bound_degrades
        && bound_clean_input
        && no_clean_unbound
        && no_clean_lost_fidelity)
    {
        violations
            .push(M5InputCredentialRegistriesViolation::CompositionAndShortcutsDoNotFightNotProven);
    }

    // AC3: credential-store copy remains truthful, privacy-safe, and platform-correct without leaking false
    // certainty or hidden storage downgrades. Clean credential entries cover every canonical settings / auth /
    // support surface with full presentation-form coverage while providing the disclosure triple, an
    // untruthful-or-leaky example degrades, a phrasing-incomplete example degrades, and no clean credential
    // entry is missing the triple.
    let clean_wording_surfaces: BTreeSet<String> = credentials()
        .filter(|ex| {
            ex.is_clean()
                && ex.wording_surface_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.covers_all_presentation_forms
        })
        .map(|ex| ex.wording_surface.clone())
        .collect();
    let wording_surfaces_covered = M5CredentialWordingSurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_wording_surfaces.contains(s.as_str()));
    let untruthful_degrades = credentials().any(|ex| {
        ex.degrade_reason
            == Some(M5CredentialStoreWordingEntryDegradeReason::StorageClaimUntruthfulOrLeaky)
    });
    let phrasing_incomplete_degrades = credentials().any(|ex| {
        ex.degrade_reason
            == Some(M5CredentialStoreWordingEntryDegradeReason::WordingPhrasingCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !credentials().any(|ex| ex.is_clean() && !ex.provides_complete_disclosure_triple);
    if !(wording_surfaces_covered
        && untruthful_degrades
        && phrasing_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5InputCredentialRegistriesViolation::CredentialCopyTruthfulAndPrivacySafeNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The platform-fit families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5PlatformFitFamily; 2] = [
    M5PlatformFitFamily::InputMethod,
    M5PlatformFitFamily::CredentialStoreWording,
];
