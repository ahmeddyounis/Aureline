//! Typed generated-artifact descriptors consumed by the M5 file-tree,
//! breadcrumb, search, diff/review, AI-context, and support/export surfaces.
//!
//! The sibling [`crate::m5_generated_governance`] matrix certifies
//! generated-artifact truth one row per *class*. This module models the
//! per-*artifact* object those surfaces actually render: one
//! [`GeneratedArtifactDescriptor`] per generated file, carrying its
//! canonical-source reference, its generator/version
//! [`GeneratorIdentity`], its [`AuthorityClass`] provenance class, its
//! [`DriftState`], its declared writable-boundary [`EditPosture`], its
//! regeneration route, and its reversible-checkpoint lineage reference.
//!
//! A single [`derive_descriptor_presentation`] engine folds those fields
//! into one [`DescriptorPresentation`]: the [`PresentedAuthority`] a
//! surface may show, whether an *ordinary source* claim is allowed, the
//! narrowed [`EditPosture`], and the stable block-reason tokens explaining
//! any downgrade. The marquee guardrail is frozen here: **hidden or missing
//! canonical-source information blocks any ordinary-source claim** on the
//! affected artifact, so a derived file is never presented as an ordinary
//! authoritative source merely because it looks like a file on disk.
//!
//! Every surface reads the *same* [`IdentityFields`] through
//! [`GeneratedArtifactDescriptor::project`], so the file tree, a search
//! result, a review view, an AI context line, and a support export cannot
//! disagree about a file's authority, generator, drift, or edit posture.
//! [`GeneratedArtifactDescriptor::copy_line`] is the one stable copy/export
//! form diagnostics and docs cite, so there is one object model rather than
//! a lossy text-only summary.
//!
//! The packet is mirrored, byte-for-byte, by the checked-in schema,
//! reviewer doc, proof packet, certification report, and fixture corpus
//! named on the module constants, so release, support, docs, and help
//! consume one source of truth.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/generated/generated-artifact-descriptor.schema.json`](../../../../schemas/generated/generated-artifact-descriptor.schema.json)
//! - [`/docs/generated/generated-artifact-descriptor.md`](../../../../docs/generated/generated-artifact-descriptor.md)
//! - [`/artifacts/generated/generated-artifact-descriptor-packet.json`](../../../../artifacts/generated/generated-artifact-descriptor-packet.json)
//! - [`/artifacts/generated/generated-artifact-descriptor.md`](../../../../artifacts/generated/generated-artifact-descriptor.md)
//! - [`/fixtures/generated/generated-artifact-descriptor/`](../../../../fixtures/generated/generated-artifact-descriptor/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_generated_governance::{ArtifactClass, AuthorityClass, EditPosture};

/// Schema version stamped onto the descriptor packet and fixtures.
pub const GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the descriptor packet.
pub const GENERATED_ARTIFACT_DESCRIPTOR_PACKET_RECORD_KIND: &str =
    "generated_artifact_descriptor_packet_record";

/// Stable record-kind tag carried by descriptor fixtures.
pub const GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_RECORD_KIND: &str =
    "generated_artifact_descriptor_fixture_record";

/// Stable packet id every surface binding ingests.
pub const GENERATED_ARTIFACT_DESCRIPTOR_PACKET_ID: &str =
    "generated.generated_artifact_descriptor.v1";

/// Repo-relative schema ref.
pub const GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/generated/generated-artifact-descriptor.schema.json";

/// Repo-relative reviewer doc ref.
pub const GENERATED_ARTIFACT_DESCRIPTOR_DOC_REF: &str =
    "docs/generated/generated-artifact-descriptor.md";

/// Repo-relative machine-readable proof packet.
pub const GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF: &str =
    "artifacts/generated/generated-artifact-descriptor-packet.json";

/// Repo-relative reviewer certification summary.
pub const GENERATED_ARTIFACT_DESCRIPTOR_REPORT_REF: &str =
    "artifacts/generated/generated-artifact-descriptor.md";

/// Repo-relative fixture directory.
pub const GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_DIR: &str =
    "fixtures/generated/generated-artifact-descriptor";

/// Repo-relative fixture manifest.
pub const GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_MANIFEST_REF: &str =
    "fixtures/generated/generated-artifact-descriptor/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// The generator family that produced an artifact. Naming the producer is
/// the first half of generator identity; the version completes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorKind {
    /// A project or file scaffolded from a template or starter.
    Template,
    /// A notebook kernel that captured cell output.
    Kernel,
    /// A preview/runtime builder that emitted a derivative.
    Builder,
    /// A request/runner that captured a response artifact.
    Runner,
    /// A framework code generator.
    Framework,
    /// An AI-assisted composer that produced an edit.
    Composer,
    /// An exporter that projected a support packet.
    Exporter,
}

impl GeneratorKind {
    /// Every generator family in canonical order.
    pub const ALL: [Self; 7] = [
        Self::Template,
        Self::Kernel,
        Self::Builder,
        Self::Runner,
        Self::Framework,
        Self::Composer,
        Self::Exporter,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Template => "template",
            Self::Kernel => "kernel",
            Self::Builder => "builder",
            Self::Runner => "runner",
            Self::Framework => "framework",
            Self::Composer => "composer",
            Self::Exporter => "exporter",
        }
    }
}

/// How an artifact's canonical source is linked and surfaced. Declaration
/// order is the narrowing order: [`CanonicalSourceState::Linked`] is the
/// strongest state and [`CanonicalSourceState::Missing`] the weakest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalSourceState {
    /// The canonical source is recorded and visible, so the artifact can be
    /// inspected and diffed against the source it derives from.
    Linked,
    /// A canonical source exists but is hidden from the user — outside the
    /// workspace, redacted, or otherwise not surfaceable — so its linkage
    /// cannot be vouched for.
    Hidden,
    /// No canonical source is recorded for the artifact at all.
    Missing,
}

impl CanonicalSourceState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Hidden => "hidden",
            Self::Missing => "missing",
        }
    }

    /// Whether this state hides or drops the canonical-source linkage. The
    /// frozen guardrail: a hidden or missing canonical source blocks any
    /// ordinary-source claim on the artifact.
    pub const fn blocks_ordinary_source(self) -> bool {
        matches!(self, Self::Hidden | Self::Missing)
    }

    /// The writable-boundary floor this state forces, if any. A hidden
    /// source cannot prove a direct edit survives regeneration, so it caps
    /// the posture at a reviewed override; a missing source has no source to
    /// regenerate against, so it forces a regenerate-only boundary.
    pub const fn edit_posture_floor(self) -> Option<EditPosture> {
        match self {
            Self::Linked => None,
            Self::Hidden => Some(EditPosture::ReviewedOverrideRequired),
            Self::Missing => Some(EditPosture::RegenerateOnly),
        }
    }

    /// The stable block-reason token this state contributes, if any.
    pub const fn block_token(self) -> Option<&'static str> {
        match self {
            Self::Linked => None,
            Self::Hidden => Some("canonical_source_hidden"),
            Self::Missing => Some("canonical_source_missing"),
        }
    }
}

/// Whether an artifact's derived bytes still match their canonical source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftState {
    /// The derived bytes match the canonical source.
    InSync,
    /// The derived bytes have diverged from the canonical source.
    Drifting,
    /// Drift cannot be computed because the canonical source is absent.
    SourceMissing,
    /// Drift has not been computed yet.
    Unknown,
}

impl DriftState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::Drifting => "drifting",
            Self::SourceMissing => "source_missing",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this drift state leaves the artifact's relation to its source
    /// uncertain, which withholds the ordinary-source or annotated-derived
    /// presentation in favor of a provenance-withheld one.
    pub const fn is_uncertain(self) -> bool {
        matches!(self, Self::SourceMissing | Self::Unknown)
    }

    /// The writable-boundary floor this drift state forces, if any. Drifting
    /// bytes risk a clobbered edit, so they require a reviewed override; a
    /// missing source forces a regenerate-only boundary; unknown drift
    /// cannot prove an edit is safe, so it requires a reviewed override.
    pub const fn edit_posture_floor(self) -> Option<EditPosture> {
        match self {
            Self::InSync => None,
            Self::Drifting => Some(EditPosture::ReviewedOverrideRequired),
            Self::SourceMissing => Some(EditPosture::RegenerateOnly),
            Self::Unknown => Some(EditPosture::ReviewedOverrideRequired),
        }
    }

    /// The stable block-reason token this drift state contributes, if any.
    pub const fn block_token(self) -> Option<&'static str> {
        match self {
            Self::InSync => None,
            Self::Drifting => Some("drift_drifting"),
            Self::SourceMissing => Some("drift_source_missing"),
            Self::Unknown => Some("drift_unknown"),
        }
    }
}

/// How a surface may present an artifact once provenance is folded in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentedAuthority {
    /// The artifact may be presented as ordinary authoritative source: it is
    /// canonical-authoritative, its source is linked, and it is in sync.
    OrdinarySource,
    /// The artifact must be presented as a derived file with provenance
    /// annotation; it is never shown as ordinary source.
    DerivedAnnotated,
    /// Provenance is incomplete — hidden/missing source or uncertain
    /// drift — so the artifact is presented with provenance withheld rather
    /// than as ordinary or annotated-derived.
    ProvenanceWithheld,
}

impl PresentedAuthority {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinarySource => "ordinary_source",
            Self::DerivedAnnotated => "derived_annotated",
            Self::ProvenanceWithheld => "provenance_withheld",
        }
    }

    /// A short surface-agnostic label for the presentation.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::OrdinarySource => "source",
            Self::DerivedAnnotated => "derived",
            Self::ProvenanceWithheld => "provenance withheld",
        }
    }
}

/// A surface that renders generated-artifact identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// The workspace file tree.
    FileTree,
    /// A search result row.
    SearchResult,
    /// A diff/review view.
    ReviewView,
    /// An AI prompt-context attachment line.
    AiContext,
    /// A metadata-first support export.
    SupportExport,
}

impl SurfaceKind {
    /// Every rendered surface in canonical order.
    pub const ALL: [Self; 5] = [
        Self::FileTree,
        Self::SearchResult,
        Self::ReviewView,
        Self::AiContext,
        Self::SupportExport,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileTree => "file_tree",
            Self::SearchResult => "search_result",
            Self::ReviewView => "review_view",
            Self::AiContext => "ai_context",
            Self::SupportExport => "support_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Descriptor fields.
// ---------------------------------------------------------------------------

/// The generator that produced an artifact, with its version. Both the name
/// and the version are required: a generator without a version cannot prove
/// the artifact was produced by a known, reproducible generator run.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GeneratorIdentity {
    /// Generator family.
    pub kind: GeneratorKind,
    /// Review-safe generator name (template, kernel, builder, framework,
    /// composer, or exporter identifier).
    pub name: String,
    /// Generator version string.
    pub version: String,
}

impl GeneratorIdentity {
    /// The stable `kind/name@version` copy form for the generator identity.
    pub fn copy_form(&self) -> String {
        format!("{}/{}@{}", self.kind.as_str(), self.name, self.version)
    }
}

/// A reference to the canonical source an artifact derives from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalSourceRef {
    /// How the canonical source is linked and surfaced.
    pub state: CanonicalSourceState,
    /// Review-safe reference to the canonical source. Non-empty when the
    /// source is [`CanonicalSourceState::Linked`]; empty when it is hidden
    /// or missing.
    pub source_ref: String,
}

/// The computed conclusion the descriptor presents to every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorPresentation {
    /// How a surface may present the artifact.
    pub presented_authority: PresentedAuthority,
    /// Whether an ordinary-source claim is allowed. Always false when the
    /// canonical source is hidden or missing.
    pub ordinary_source_claim_allowed: bool,
    /// The narrowed writable-boundary posture after folding in source state
    /// and drift.
    pub effective_edit_posture: EditPosture,
    /// True when the writable-boundary posture narrowed below the declared
    /// one.
    pub edit_posture_downgraded: bool,
    /// Stable tokens naming every input that blocked the ordinary-source
    /// claim or narrowed the edit posture, sorted and deduplicated.
    pub block_reason_tokens: Vec<String>,
    /// The one stable copy/export form for the descriptor.
    pub copy_line: String,
}

/// The identity fields every surface must display verbatim. Computing them
/// once and projecting them into each surface is what keeps the file tree,
/// search, review, AI context, and support export from disagreeing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityFields {
    /// Generated-artifact class.
    pub artifact_class: ArtifactClass,
    /// Provenance/authority class of the bytes relative to the source.
    pub authority_class: AuthorityClass,
    /// Generator that produced the artifact, with version.
    pub generator: GeneratorIdentity,
    /// Canonical-source state.
    pub canonical_source_state: CanonicalSourceState,
    /// Drift between the derived bytes and the canonical source.
    pub drift_state: DriftState,
    /// How a surface may present the artifact.
    pub presented_authority: PresentedAuthority,
    /// Narrowed writable-boundary posture.
    pub effective_edit_posture: EditPosture,
    /// Whether an ordinary-source claim is allowed.
    pub ordinary_source_claim_allowed: bool,
}

/// The stable names of the identity fields a surface preserves verbatim.
pub const IDENTITY_FIELD_NAMES: [&str; 8] = [
    "artifact_class",
    "authority_class",
    "generator",
    "canonical_source_state",
    "drift_state",
    "presented_authority",
    "effective_edit_posture",
    "ordinary_source_claim_allowed",
];

/// One surface's projection of a descriptor: surface-appropriate prose plus
/// the shared identity fields and the stable copy form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceProjection {
    /// Surface this projection targets.
    pub surface: SurfaceKind,
    /// Shared identity fields, identical across every surface.
    pub identity: IdentityFields,
    /// Short surface badge.
    pub badge: String,
    /// One-line surface headline.
    pub headline: String,
    /// Surface-appropriate detail line.
    pub detail: String,
    /// The stable copy/export form, identical across every surface.
    pub copy_line: String,
}

/// One typed generated-artifact descriptor: the per-file object the M5
/// surfaces render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactDescriptor {
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Generated-artifact class.
    pub artifact_class: ArtifactClass,
    /// Review-safe display label for the artifact path.
    pub artifact_path_label: String,
    /// Provenance/authority class of the bytes relative to the source.
    pub authority_class: AuthorityClass,
    /// Generator that produced the artifact, with version.
    pub generator: GeneratorIdentity,
    /// Canonical source the artifact derives from.
    pub canonical_source: CanonicalSourceRef,
    /// Review-safe regeneration route that rebuilds the artifact.
    pub regeneration_route: String,
    /// Drift between the derived bytes and the canonical source.
    pub drift_state: DriftState,
    /// Writable-boundary posture declared for the artifact before narrowing.
    pub declared_edit_posture: EditPosture,
    /// Reference to the reversible-checkpoint lineage that captured the
    /// change.
    pub checkpoint_lineage_ref: String,
    /// Upstream generated-artifact packets backing this descriptor.
    pub evidence_refs: Vec<String>,
    /// Review-safe "why this artifact" inspector line.
    pub why_this_artifact: String,
    /// Computed presentation stamped onto the descriptor.
    pub presentation: DescriptorPresentation,
    /// Short reviewer note.
    pub notes: String,
}

impl GeneratedArtifactDescriptor {
    /// The identity fields every surface must display for this descriptor.
    pub fn identity_fields(&self) -> IdentityFields {
        IdentityFields {
            artifact_class: self.artifact_class,
            authority_class: self.authority_class,
            generator: self.generator.clone(),
            canonical_source_state: self.canonical_source.state,
            drift_state: self.drift_state,
            presented_authority: self.presentation.presented_authority,
            effective_edit_posture: self.presentation.effective_edit_posture,
            ordinary_source_claim_allowed: self.presentation.ordinary_source_claim_allowed,
        }
    }

    /// The one stable copy/export form for the descriptor.
    pub fn copy_line(&self) -> String {
        descriptor_copy_line(self)
    }

    /// Projects this descriptor onto one surface, embedding the shared
    /// identity fields so every surface renders the same truth.
    pub fn project(&self, surface: SurfaceKind) -> SurfaceProjection {
        let identity = self.identity_fields();
        SurfaceProjection {
            surface,
            badge: surface_badge(self, surface),
            headline: surface_headline(self, surface),
            detail: surface_detail(self, surface),
            copy_line: self.presentation.copy_line.clone(),
            identity,
        }
    }

    /// Projects this descriptor onto every rendered surface in canonical
    /// order.
    pub fn project_all(&self) -> Vec<SurfaceProjection> {
        SurfaceKind::ALL
            .into_iter()
            .map(|surface| self.project(surface))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Presentation engine: the single source of truth for the conclusion.
// ---------------------------------------------------------------------------

/// Derives the presentation conclusion for an artifact from its provenance
/// inputs.
///
/// This is the canonical engine the descriptors, the surface projections,
/// the fixtures, and the consuming surfaces all share. Two guardrails are
/// frozen here:
///
/// - **Hidden or missing canonical source blocks the ordinary-source
///   claim.** An artifact is presented as [`PresentedAuthority::OrdinarySource`]
///   only when it is canonical-authoritative, its source is
///   [`CanonicalSourceState::Linked`], and it is [`DriftState::InSync`].
/// - **The writable boundary only narrows.** The effective posture starts
///   at the declared posture and is floored by the canonical-source state
///   and the drift state; the strictest result wins and the posture is never
///   widened.
pub fn derive_descriptor_presentation(
    artifact_class: ArtifactClass,
    authority_class: AuthorityClass,
    generator: &GeneratorIdentity,
    canonical_source_state: CanonicalSourceState,
    drift_state: DriftState,
    declared_edit_posture: EditPosture,
) -> DescriptorPresentation {
    let presented_authority = match authority_class {
        AuthorityClass::CanonicalAuthoritative => {
            if canonical_source_state == CanonicalSourceState::Linked
                && drift_state == DriftState::InSync
            {
                PresentedAuthority::OrdinarySource
            } else {
                PresentedAuthority::ProvenanceWithheld
            }
        }
        AuthorityClass::DerivedEditable | AuthorityClass::DerivedReadonly => {
            if canonical_source_state == CanonicalSourceState::Linked && !drift_state.is_uncertain()
            {
                PresentedAuthority::DerivedAnnotated
            } else {
                PresentedAuthority::ProvenanceWithheld
            }
        }
    };

    let ordinary_source_claim_allowed = presented_authority == PresentedAuthority::OrdinarySource;

    let mut effective_edit_posture = declared_edit_posture;
    if let Some(floor) = canonical_source_state.edit_posture_floor() {
        if floor.severity() > effective_edit_posture.severity() {
            effective_edit_posture = floor;
        }
    }
    if let Some(floor) = drift_state.edit_posture_floor() {
        if floor.severity() > effective_edit_posture.severity() {
            effective_edit_posture = floor;
        }
    }

    let mut block_reason_tokens = Vec::new();
    if let Some(token) = canonical_source_state.block_token() {
        block_reason_tokens.push(token.to_owned());
    }
    if let Some(token) = drift_state.block_token() {
        block_reason_tokens.push(token.to_owned());
    }
    block_reason_tokens.sort();
    block_reason_tokens.dedup();

    let copy_line = copy_line_for(
        artifact_class,
        authority_class,
        generator,
        canonical_source_state,
        drift_state,
        presented_authority,
        effective_edit_posture,
        ordinary_source_claim_allowed,
    );

    DescriptorPresentation {
        presented_authority,
        ordinary_source_claim_allowed,
        effective_edit_posture,
        edit_posture_downgraded: effective_edit_posture.severity()
            > declared_edit_posture.severity(),
        block_reason_tokens,
        copy_line,
    }
}

/// Computes the stable copy/export form for a descriptor.
pub fn descriptor_copy_line(descriptor: &GeneratedArtifactDescriptor) -> String {
    copy_line_for(
        descriptor.artifact_class,
        descriptor.authority_class,
        &descriptor.generator,
        descriptor.canonical_source.state,
        descriptor.drift_state,
        descriptor.presentation.presented_authority,
        descriptor.presentation.effective_edit_posture,
        descriptor.presentation.ordinary_source_claim_allowed,
    )
}

#[allow(clippy::too_many_arguments)]
fn copy_line_for(
    artifact_class: ArtifactClass,
    authority_class: AuthorityClass,
    generator: &GeneratorIdentity,
    canonical_source_state: CanonicalSourceState,
    drift_state: DriftState,
    presented_authority: PresentedAuthority,
    effective_edit_posture: EditPosture,
    ordinary_source_claim_allowed: bool,
) -> String {
    format!(
        "generated-artifact class={} authority={} generator={} source={} drift={} presented={} edit={} ordinary_source={}",
        artifact_class.as_str(),
        authority_class.as_str(),
        generator.copy_form(),
        canonical_source_state.as_str(),
        drift_state.as_str(),
        presented_authority.as_str(),
        effective_edit_posture.as_str(),
        ordinary_source_claim_allowed,
    )
}

// ---------------------------------------------------------------------------
// Surface projection prose.
// ---------------------------------------------------------------------------

fn surface_badge(descriptor: &GeneratedArtifactDescriptor, surface: SurfaceKind) -> String {
    let presented = descriptor.presentation.presented_authority;
    match surface {
        SurfaceKind::FileTree | SurfaceKind::SearchResult => match presented {
            PresentedAuthority::OrdinarySource => "Generated".to_owned(),
            PresentedAuthority::DerivedAnnotated => "Derived".to_owned(),
            PresentedAuthority::ProvenanceWithheld => "Provenance withheld".to_owned(),
        },
        SurfaceKind::ReviewView => match descriptor.presentation.effective_edit_posture {
            EditPosture::DirectEditAllowed => "Direct edit".to_owned(),
            EditPosture::ReviewedOverrideRequired => "Reviewed override".to_owned(),
            EditPosture::RegenerateOnly => "Regenerate only".to_owned(),
        },
        SurfaceKind::AiContext => "Generated context".to_owned(),
        SurfaceKind::SupportExport => "Generated artifact".to_owned(),
    }
}

fn surface_headline(descriptor: &GeneratedArtifactDescriptor, surface: SurfaceKind) -> String {
    let class = descriptor.artifact_class.as_str();
    let presented = descriptor.presentation.presented_authority.short_label();
    match surface {
        SurfaceKind::FileTree => {
            format!(
                "{} · {} · {}",
                descriptor.artifact_path_label, class, presented
            )
        }
        SurfaceKind::SearchResult => {
            format!(
                "{} ({}, {})",
                descriptor.artifact_path_label, class, presented
            )
        }
        SurfaceKind::ReviewView => format!(
            "{} — {} edit boundary",
            descriptor.artifact_path_label,
            descriptor.presentation.effective_edit_posture.as_str()
        ),
        SurfaceKind::AiContext => format!(
            "{} is a generated {} artifact ({})",
            descriptor.artifact_path_label, class, presented
        ),
        SurfaceKind::SupportExport => descriptor.presentation.copy_line.clone(),
    }
}

fn surface_detail(descriptor: &GeneratedArtifactDescriptor, surface: SurfaceKind) -> String {
    let generator = descriptor.generator.copy_form();
    let drift = descriptor.drift_state.as_str();
    let source = descriptor.canonical_source.state.as_str();
    match surface {
        SurfaceKind::FileTree | SurfaceKind::SearchResult => {
            format!("Generated by {generator}; canonical source {source}, drift {drift}.")
        }
        SurfaceKind::ReviewView => {
            if descriptor.presentation.block_reason_tokens.is_empty() {
                format!(
                    "Direct edits follow the {} boundary; regenerate via {}.",
                    descriptor.presentation.effective_edit_posture.as_str(),
                    descriptor.regeneration_route
                )
            } else {
                format!(
                    "Edit boundary narrowed to {} ({}); regenerate via {}.",
                    descriptor.presentation.effective_edit_posture.as_str(),
                    descriptor.presentation.block_reason_tokens.join(", "),
                    descriptor.regeneration_route
                )
            }
        }
        SurfaceKind::AiContext => format!(
            "Treat as generated by {generator}; not ordinary source unless presented={}. Edit boundary: {}.",
            PresentedAuthority::OrdinarySource.as_str(),
            descriptor.presentation.effective_edit_posture.as_str()
        ),
        SurfaceKind::SupportExport => descriptor.why_this_artifact.clone(),
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One binding proving a surface ingests this packet rather than
/// re-deriving generated-artifact identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorSurfaceBinding {
    /// Surface that ingests the packet.
    pub surface: SurfaceKind,
    /// Checked consumer ref that renders the descriptor.
    pub consumer_ref: String,
    /// Packet id the surface ingests.
    pub ingested_packet_id: String,
    /// Identity fields the surface preserves verbatim.
    pub preserved_identity_fields: Vec<String>,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Shared source references for the descriptor packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorSourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Proof packet ref.
    pub packet_ref: String,
    /// Certification summary ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet modeling typed generated-artifact descriptors and the
/// surfaces that render them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactDescriptorPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: DescriptorSourceContractRefs,
    /// Surfaces that render the descriptor.
    pub surfaces: Vec<SurfaceKind>,
    /// Upstream generated-artifact packets this lane composes.
    pub evidence_packet_refs: Vec<String>,
    /// Descriptors, one per generated-artifact class.
    pub descriptors: Vec<GeneratedArtifactDescriptor>,
    /// Surface bindings, one per rendered surface.
    pub surface_bindings: Vec<DescriptorSurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a descriptor to its expected presentation, proving
/// the canonical presentation behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactDescriptorFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The descriptor under test.
    pub descriptor: GeneratedArtifactDescriptor,
    /// Expected presented authority.
    pub expected_presented_authority: PresentedAuthority,
    /// Expected ordinary-source claim.
    pub expected_ordinary_source_claim_allowed: bool,
    /// Expected effective writable-boundary posture.
    pub expected_effective_edit_posture: EditPosture,
    /// Expected block-reason tokens.
    pub expected_block_reason_tokens: Vec<String>,
    /// One consumer that renders this descriptor.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the descriptor packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "generated-artifact descriptor validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// ---------------------------------------------------------------------------
// Evidence-packet vocabulary used by the seed.
// ---------------------------------------------------------------------------

const GOVERNANCE_PACKET_REF: &str = "artifacts/generated/m5-generated-proof-packet.json";
const SCAFFOLD_LINEAGE_REF: &str =
    "artifacts/scaffold/stabilize-template-manifest-scaffold-lineage.md";
const TEMPLATE_HEALTH_REF: &str = "artifacts/scaffolding/template_health_states.yaml";
const NOTEBOOK_LINEAGE_REF: &str =
    "artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json";
const SAVE_REVIEW_REF: &str = "artifacts/fs/save_review_choice_matrix.yaml";
const MUTATION_CLASSES_REF: &str = "artifacts/change/mutation_classes.yaml";
const ROLLBACK_CHECKPOINT_REF: &str =
    "artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml";
const RESTORE_PROVENANCE_REF: &str = "artifacts/migration/m3/restore_provenance_packet.md";

fn evidence_packet_refs() -> Vec<String> {
    [
        GOVERNANCE_PACKET_REF,
        SCAFFOLD_LINEAGE_REF,
        TEMPLATE_HEALTH_REF,
        NOTEBOOK_LINEAGE_REF,
        SAVE_REVIEW_REF,
        MUTATION_CLASSES_REF,
        ROLLBACK_CHECKPOINT_REF,
        RESTORE_PROVENANCE_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// The canonical generator family for a class when produced normally.
fn class_generator_kind(artifact_class: ArtifactClass) -> GeneratorKind {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => GeneratorKind::Template,
        ArtifactClass::NotebookOutput => GeneratorKind::Kernel,
        ArtifactClass::PreviewDerivative => GeneratorKind::Builder,
        ArtifactClass::RequestArtifact => GeneratorKind::Runner,
        ArtifactClass::FrameworkCodegen => GeneratorKind::Framework,
        ArtifactClass::AiAssistedEdit => GeneratorKind::Composer,
        ArtifactClass::SupportPacket => GeneratorKind::Exporter,
    }
}

/// The canonical authority class and declared edit posture for a class.
fn class_authority(artifact_class: ArtifactClass) -> (AuthorityClass, EditPosture) {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => (
            AuthorityClass::CanonicalAuthoritative,
            EditPosture::DirectEditAllowed,
        ),
        ArtifactClass::NotebookOutput => {
            (AuthorityClass::DerivedReadonly, EditPosture::RegenerateOnly)
        }
        ArtifactClass::PreviewDerivative => {
            (AuthorityClass::DerivedReadonly, EditPosture::RegenerateOnly)
        }
        ArtifactClass::RequestArtifact => (
            AuthorityClass::DerivedEditable,
            EditPosture::ReviewedOverrideRequired,
        ),
        ArtifactClass::FrameworkCodegen => (
            AuthorityClass::DerivedEditable,
            EditPosture::ReviewedOverrideRequired,
        ),
        ArtifactClass::AiAssistedEdit => (
            AuthorityClass::CanonicalAuthoritative,
            EditPosture::DirectEditAllowed,
        ),
        ArtifactClass::SupportPacket => {
            (AuthorityClass::DerivedReadonly, EditPosture::RegenerateOnly)
        }
    }
}

fn class_generator(artifact_class: ArtifactClass) -> GeneratorIdentity {
    let (name, version) = match artifact_class {
        ArtifactClass::ScaffoldedProject => ("rust-cli-starter", "1.4.0"),
        ArtifactClass::NotebookOutput => ("python-kernel", "3.11.6"),
        ArtifactClass::PreviewDerivative => ("preview-bundler", "0.9.2"),
        ArtifactClass::RequestArtifact => ("request-runner", "2.3.1"),
        ArtifactClass::FrameworkCodegen => ("openapi-codegen", "5.0.0"),
        ArtifactClass::AiAssistedEdit => ("scoped-composer", "1.0.0"),
        ArtifactClass::SupportPacket => ("support-exporter", "4.2.0"),
    };
    GeneratorIdentity {
        kind: class_generator_kind(artifact_class),
        name: name.to_owned(),
        version: version.to_owned(),
    }
}

fn class_path_label(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "src/main.rs",
        ArtifactClass::NotebookOutput => "analysis.ipynb#cell-7-output",
        ArtifactClass::PreviewDerivative => ".preview/bundle.js",
        ArtifactClass::RequestArtifact => "requests/users.list.response.json",
        ArtifactClass::FrameworkCodegen => "generated/api_client.rs",
        ArtifactClass::AiAssistedEdit => "src/parser.rs",
        ArtifactClass::SupportPacket => "support/diagnostic-bundle.json",
    }
}

fn class_source_ref(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "templates/rust-cli-starter",
        ArtifactClass::NotebookOutput => "analysis.ipynb#cell-7",
        ArtifactClass::PreviewDerivative => "src/index.ts",
        ArtifactClass::RequestArtifact => "requests/users.list.request.json",
        ArtifactClass::FrameworkCodegen => "openapi/users.yaml",
        ArtifactClass::AiAssistedEdit => "src/parser.rs@checkpoint",
        ArtifactClass::SupportPacket => "workspace diagnostics snapshot",
    }
}

fn class_regeneration_route(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "re-run the project scaffold from its template",
        ArtifactClass::NotebookOutput => "re-run the notebook cell from its kernel",
        ArtifactClass::PreviewDerivative => "rebuild the preview bundle from source",
        ArtifactClass::RequestArtifact => "replay the saved request",
        ArtifactClass::FrameworkCodegen => "re-run the framework code generator",
        ArtifactClass::AiAssistedEdit => "re-run the scoped AI apply from its checkpoint",
        ArtifactClass::SupportPacket => "re-export the support packet",
    }
}

fn class_why_this_artifact(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => {
            "This file was scaffolded from a template; once written it is the canonical source you edit directly, with the template recorded as its origin and a regeneration route back to the scaffold."
        }
        ArtifactClass::NotebookOutput => {
            "This is captured notebook cell output; it is derived from its cell and regenerated by re-running the kernel rather than hand-edited."
        }
        ArtifactClass::PreviewDerivative => {
            "This is a preview/runtime derivative built from source; it is rebuilt from its source rather than edited in place."
        }
        ArtifactClass::RequestArtifact => {
            "This is a captured request response; it is derived from a saved request and may be annotated through a reviewed override."
        }
        ArtifactClass::FrameworkCodegen => {
            "This is framework-generated code; hand edits cross a generator boundary and escalate through a reviewed override before the next regeneration."
        }
        ArtifactClass::AiAssistedEdit => {
            "This edit was produced by a scoped AI apply; once accepted it is canonical source you own, with the apply checkpoint recorded for rollback."
        }
        ArtifactClass::SupportPacket => {
            "This is an exported support packet; it is a derived projection regenerated by re-export rather than edited directly."
        }
    }
}

fn class_consumer_ref(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => {
            "crates/aureline-workspace/src/generated_artifacts/mod.rs"
        }
        ArtifactClass::NotebookOutput => "crates/aureline-search/src/results/mod.rs",
        ArtifactClass::PreviewDerivative => "crates/aureline-review/src/change_inspector/mod.rs",
        ArtifactClass::RequestArtifact => "crates/aureline-ai/src/context_inspector/mod.rs",
        ArtifactClass::FrameworkCodegen => "crates/aureline-review/src/change_inspector/mod.rs",
        ArtifactClass::AiAssistedEdit => "crates/aureline-ai/src/context_inspector/mod.rs",
        ArtifactClass::SupportPacket => "crates/aureline-support/src/generated_lineage/mod.rs",
    }
}

fn class_checkpoint_lineage_ref(_artifact_class: ArtifactClass) -> &'static str {
    ROLLBACK_CHECKPOINT_REF
}

fn class_evidence_refs(artifact_class: ArtifactClass) -> Vec<String> {
    let class_ref = match artifact_class {
        ArtifactClass::ScaffoldedProject => SCAFFOLD_LINEAGE_REF,
        ArtifactClass::NotebookOutput => NOTEBOOK_LINEAGE_REF,
        ArtifactClass::PreviewDerivative => NOTEBOOK_LINEAGE_REF,
        ArtifactClass::RequestArtifact => SAVE_REVIEW_REF,
        ArtifactClass::FrameworkCodegen => MUTATION_CLASSES_REF,
        ArtifactClass::AiAssistedEdit => RESTORE_PROVENANCE_REF,
        ArtifactClass::SupportPacket => RESTORE_PROVENANCE_REF,
    };
    let mut refs: BTreeSet<String> = BTreeSet::new();
    refs.insert(GOVERNANCE_PACKET_REF.to_owned());
    refs.insert(class_ref.to_owned());
    refs.insert(class_checkpoint_lineage_ref(artifact_class).to_owned());
    refs.into_iter().collect()
}

/// Builds a descriptor for a class with explicit source state and drift,
/// stamping the engine-computed presentation onto it.
fn build_descriptor(
    descriptor_id: &str,
    artifact_class: ArtifactClass,
    canonical_source_state: CanonicalSourceState,
    drift_state: DriftState,
    notes: &str,
) -> GeneratedArtifactDescriptor {
    let (authority_class, declared_edit_posture) = class_authority(artifact_class);
    let generator = class_generator(artifact_class);
    let source_ref = match canonical_source_state {
        CanonicalSourceState::Linked => class_source_ref(artifact_class).to_owned(),
        CanonicalSourceState::Hidden | CanonicalSourceState::Missing => String::new(),
    };
    let presentation = derive_descriptor_presentation(
        artifact_class,
        authority_class,
        &generator,
        canonical_source_state,
        drift_state,
        declared_edit_posture,
    );
    GeneratedArtifactDescriptor {
        descriptor_id: descriptor_id.to_owned(),
        artifact_class,
        artifact_path_label: class_path_label(artifact_class).to_owned(),
        authority_class,
        generator,
        canonical_source: CanonicalSourceRef {
            state: canonical_source_state,
            source_ref,
        },
        regeneration_route: class_regeneration_route(artifact_class).to_owned(),
        drift_state,
        declared_edit_posture,
        checkpoint_lineage_ref: class_checkpoint_lineage_ref(artifact_class).to_owned(),
        evidence_refs: class_evidence_refs(artifact_class),
        why_this_artifact: class_why_this_artifact(artifact_class).to_owned(),
        presentation,
        notes: notes.to_owned(),
    }
}

/// A healthy descriptor for a class: source linked, in sync.
fn healthy_descriptor(artifact_class: ArtifactClass) -> GeneratedArtifactDescriptor {
    build_descriptor(
        &format!("generated.descriptor.{}", artifact_class.as_str()),
        artifact_class,
        CanonicalSourceState::Linked,
        DriftState::InSync,
        "Source linked and in sync; the descriptor presents at its declared authority and edit boundary.",
    )
}

fn binding(surface: SurfaceKind, consumer_ref: &str, summary: &str) -> DescriptorSurfaceBinding {
    DescriptorSurfaceBinding {
        surface,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: GENERATED_ARTIFACT_DESCRIPTOR_PACKET_ID.to_owned(),
        preserved_identity_fields: IDENTITY_FIELD_NAMES
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        summary: summary.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in generated-artifact descriptor packet this lane
/// freezes.
pub fn seeded_generated_artifact_descriptor_packet() -> GeneratedArtifactDescriptorPacket {
    let descriptors = ArtifactClass::ALL
        .into_iter()
        .map(healthy_descriptor)
        .collect();

    let surface_bindings = vec![
        binding(
            SurfaceKind::FileTree,
            "crates/aureline-workspace/src/generated_artifacts/mod.rs",
            "The workspace file tree reads the descriptor identity fields to badge a generated file and to block an ordinary-source label when its canonical source is hidden or missing.",
        ),
        binding(
            SurfaceKind::SearchResult,
            "crates/aureline-search/src/results/mod.rs",
            "Search result rows carry the same identity fields so an indexed generated file never drops its generator identity or authority class merely because it is already in the index.",
        ),
        binding(
            SurfaceKind::ReviewView,
            "crates/aureline-review/src/change_inspector/mod.rs",
            "The diff/review change inspector reuses the effective writable-boundary posture so a direct edit across a canonical-source boundary escalates through a visible reviewed override.",
        ),
        binding(
            SurfaceKind::AiContext,
            "crates/aureline-ai/src/context_inspector/mod.rs",
            "The AI context inspector attaches the descriptor identity so the model is told a file is generated, its generator, and its edit boundary instead of treating derived bytes as ordinary source.",
        ),
        binding(
            SurfaceKind::SupportExport,
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "The support export re-emits the descriptor identity and copy line with no raw paths, credentials, or generator payloads, so diagnostics cite one object model rather than a lossy text summary.",
        ),
    ];

    GeneratedArtifactDescriptorPacket {
        record_kind: GENERATED_ARTIFACT_DESCRIPTOR_PACKET_RECORD_KIND.to_owned(),
        schema_version: GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_VERSION,
        packet_id: GENERATED_ARTIFACT_DESCRIPTOR_PACKET_ID.to_owned(),
        title: "Typed generated-artifact descriptors for the M5 file-tree, search, review, AI-context, and support/export surfaces"
            .to_owned(),
        source_contract_refs: DescriptorSourceContractRefs {
            doc_ref: GENERATED_ARTIFACT_DESCRIPTOR_DOC_REF.to_owned(),
            schema_ref: GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_REF.to_owned(),
            packet_ref: GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF.to_owned(),
            report_ref: GENERATED_ARTIFACT_DESCRIPTOR_REPORT_REF.to_owned(),
            fixture_manifest_ref: GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_MANIFEST_REF.to_owned(),
        },
        surfaces: SurfaceKind::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        descriptors,
        surface_bindings,
        invariants: vec![
            "Each generated artifact carries one typed descriptor: canonical source, generator identity with version, authority class, drift state, declared writable-boundary posture, regeneration route, and reversible-checkpoint lineage.".to_owned(),
            "One presentation engine folds those fields into a presented authority, an ordinary-source claim, a narrowed writable-boundary posture, and stable block-reason tokens; the boundary only narrows, never widens.".to_owned(),
            "Hidden or missing canonical-source information blocks any ordinary-source claim on the artifact, so a derived file is never presented as ordinary authoritative source merely because it looks like a file on disk.".to_owned(),
            "Every surface — file tree, search, review, AI context, and support export — projects the same identity fields and the same copy line, so no surface drops generator identity or authority class merely because the artifact is already indexed.".to_owned(),
            "The descriptor is the one object model: support exports and docs cite the descriptor and its copy line rather than a lossy text-only summary.".to_owned(),
        ],
    }
}

/// Returns the checked-in descriptor fixture corpus this lane freezes.
pub fn seeded_generated_artifact_descriptor_fixtures() -> Vec<GeneratedArtifactDescriptorFixture> {
    let mut fixtures = Vec::new();

    // One healthy fixture per class, pinning the in-sync presentation.
    for artifact_class in ArtifactClass::ALL {
        let descriptor = healthy_descriptor(artifact_class);
        fixtures.push(fixture(
            &format!(
                "fixture.generated_artifact_descriptor.{}_in_sync",
                artifact_class.as_str()
            ),
            "Source linked and in sync",
            descriptor,
            class_consumer_ref(artifact_class),
            "A linked, in-sync artifact presents at its declared authority and edit boundary with no block-reason tokens.",
        ));
    }

    // Guardrail: a hidden canonical source blocks the ordinary-source claim
    // on a canonical-authoritative scaffolded project.
    fixtures.push(fixture(
        "fixture.generated_artifact_descriptor.scaffolded_project_source_hidden",
        "Canonical source hidden",
        build_descriptor(
            "generated.descriptor.scaffolded_project_source_hidden",
            ArtifactClass::ScaffoldedProject,
            CanonicalSourceState::Hidden,
            DriftState::Unknown,
            "A hidden canonical source blocks the ordinary-source claim and downgrades direct edits to a reviewed override.",
        ),
        class_consumer_ref(ArtifactClass::ScaffoldedProject),
        "A hidden canonical source withholds the ordinary-source presentation and narrows the edit boundary to a reviewed override.",
    ));

    // Guardrail: a missing canonical source on an AI-assisted edit blocks
    // ordinary source and forces a regenerate-only boundary.
    fixtures.push(fixture(
        "fixture.generated_artifact_descriptor.ai_assisted_edit_source_missing",
        "Canonical source missing",
        build_descriptor(
            "generated.descriptor.ai_assisted_edit_source_missing",
            ArtifactClass::AiAssistedEdit,
            CanonicalSourceState::Missing,
            DriftState::SourceMissing,
            "A missing canonical source blocks the ordinary-source claim and forces a regenerate-only boundary.",
        ),
        class_consumer_ref(ArtifactClass::AiAssistedEdit),
        "A missing canonical source withholds the ordinary-source presentation and narrows the edit boundary to regenerate-only.",
    ));

    // A drifting AI-assisted edit: still source-linked but no longer in
    // sync, so the ordinary-source claim is withheld and the boundary
    // narrows to a reviewed override.
    fixtures.push(fixture(
        "fixture.generated_artifact_descriptor.ai_assisted_edit_drifting",
        "Derived bytes drifting from source",
        build_descriptor(
            "generated.descriptor.ai_assisted_edit_drifting",
            ArtifactClass::AiAssistedEdit,
            CanonicalSourceState::Linked,
            DriftState::Drifting,
            "Drifting bytes withhold the ordinary-source claim and narrow a direct-edit boundary to a reviewed override.",
        ),
        class_consumer_ref(ArtifactClass::AiAssistedEdit),
        "Drift withholds the ordinary-source presentation and downgrades a direct-edit boundary to a reviewed override.",
    ));

    // Unknown drift on framework codegen: the derived-annotated
    // presentation is withheld until drift is computed.
    fixtures.push(fixture(
        "fixture.generated_artifact_descriptor.framework_codegen_drift_unknown",
        "Drift not yet computed",
        build_descriptor(
            "generated.descriptor.framework_codegen_drift_unknown",
            ArtifactClass::FrameworkCodegen,
            CanonicalSourceState::Linked,
            DriftState::Unknown,
            "Unknown drift withholds the annotated-derived presentation until drift is computed.",
        ),
        class_consumer_ref(ArtifactClass::FrameworkCodegen),
        "Unknown drift withholds the annotated-derived presentation and keeps the reviewed-override boundary.",
    ));

    fixtures
}

fn fixture(
    fixture_id: &str,
    scenario: &str,
    descriptor: GeneratedArtifactDescriptor,
    consumer_ref: &str,
    notes: &str,
) -> GeneratedArtifactDescriptorFixture {
    GeneratedArtifactDescriptorFixture {
        record_kind: GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        scenario: scenario.to_owned(),
        expected_presented_authority: descriptor.presentation.presented_authority,
        expected_ordinary_source_claim_allowed: descriptor
            .presentation
            .ordinary_source_claim_allowed,
        expected_effective_edit_posture: descriptor.presentation.effective_edit_posture,
        expected_block_reason_tokens: descriptor.presentation.block_reason_tokens.clone(),
        descriptor,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in descriptor packet contract.
pub fn validate_generated_artifact_descriptor_packet(
    packet: &GeneratedArtifactDescriptorPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != GENERATED_ARTIFACT_DESCRIPTOR_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != GENERATED_ARTIFACT_DESCRIPTOR_PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != GENERATED_ARTIFACT_DESCRIPTOR_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != GENERATED_ARTIFACT_DESCRIPTOR_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.surfaces != SurfaceKind::ALL.to_vec() {
        report.push(
            "packet.surfaces",
            "packet must render every surface in canonical order",
        );
    }
    if packet.evidence_packet_refs.is_empty() {
        report.push(
            "packet.evidence_packet_refs",
            "packet must cite the upstream generated-artifact evidence packets",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut covered_classes = BTreeSet::new();
    for descriptor in &packet.descriptors {
        if !covered_classes.insert(descriptor.artifact_class) {
            report.push(
                "descriptor.class_unique",
                format!("duplicate class {}", descriptor.artifact_class.as_str()),
            );
        }
        validate_descriptor(&mut report, descriptor);
    }
    for required in ArtifactClass::ALL {
        if !covered_classes.contains(&required) {
            report.push(
                "packet.covered_class",
                format!("packet must describe class {}", required.as_str()),
            );
        }
    }

    validate_surface_bindings(&mut report, packet);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_descriptor(report: &mut ValidationReport, descriptor: &GeneratedArtifactDescriptor) {
    let owner = format!("descriptor {}", descriptor.descriptor_id);

    if descriptor.descriptor_id.trim().is_empty() {
        report.push("descriptor.id", "descriptor must carry a stable id");
    }
    if descriptor.artifact_path_label.trim().is_empty() {
        report.push(
            "descriptor.path_label",
            format!("{owner} must carry an artifact path label"),
        );
    }
    if descriptor.generator.name.trim().is_empty() || descriptor.generator.version.trim().is_empty()
    {
        report.push(
            "descriptor.generator_identity",
            format!("{owner} must carry a generator name and version"),
        );
    }
    if descriptor.regeneration_route.trim().is_empty() {
        report.push(
            "descriptor.regeneration_route",
            format!("{owner} must carry a regeneration route"),
        );
    }
    if descriptor.checkpoint_lineage_ref.trim().is_empty() {
        report.push(
            "descriptor.checkpoint_lineage_ref",
            format!("{owner} must carry a checkpoint lineage ref"),
        );
    }
    if descriptor.evidence_refs.is_empty() {
        report.push(
            "descriptor.evidence_refs",
            format!("{owner} must cite at least one evidence ref"),
        );
    }
    if descriptor.why_this_artifact.trim().is_empty() {
        report.push(
            "descriptor.why_this_artifact",
            format!("{owner} must carry a why-this-artifact inspector line"),
        );
    }
    if descriptor.notes.trim().is_empty() {
        report.push("descriptor.notes", format!("{owner} must carry a note"));
    }

    // Canonical-source consistency.
    match descriptor.canonical_source.state {
        CanonicalSourceState::Linked => {
            if descriptor.canonical_source.source_ref.trim().is_empty() {
                report.push(
                    "descriptor.source_ref",
                    format!("{owner} linked canonical source must carry a source ref"),
                );
            }
        }
        CanonicalSourceState::Hidden | CanonicalSourceState::Missing => {
            if !descriptor.canonical_source.source_ref.trim().is_empty() {
                report.push(
                    "descriptor.source_ref",
                    format!("{owner} hidden/missing canonical source must not carry a source ref"),
                );
            }
        }
    }
    if descriptor.canonical_source.state == CanonicalSourceState::Missing
        && descriptor.drift_state != DriftState::SourceMissing
    {
        report.push(
            "descriptor.drift_consistency",
            format!("{owner} missing canonical source must report source_missing drift"),
        );
    }
    if descriptor.drift_state == DriftState::SourceMissing
        && descriptor.canonical_source.state != CanonicalSourceState::Missing
    {
        report.push(
            "descriptor.drift_consistency",
            format!("{owner} source_missing drift requires a missing canonical source"),
        );
    }

    // The stamped presentation must equal what the engine computes.
    let expected = derive_descriptor_presentation(
        descriptor.artifact_class,
        descriptor.authority_class,
        &descriptor.generator,
        descriptor.canonical_source.state,
        descriptor.drift_state,
        descriptor.declared_edit_posture,
    );
    if descriptor.presentation != expected {
        report.push(
            "descriptor.presentation",
            format!("{owner} stamped presentation disagrees with the engine"),
        );
    }

    // The frozen guardrail: hidden or missing canonical source blocks any
    // ordinary-source claim.
    if descriptor.canonical_source.state.blocks_ordinary_source()
        && descriptor.presentation.ordinary_source_claim_allowed
    {
        report.push(
            "descriptor.ordinary_source_guardrail",
            format!("{owner} must not allow an ordinary-source claim with hidden/missing canonical source"),
        );
    }

    // Every surface projection must carry identical identity fields and the
    // shared copy line.
    let identity = descriptor.identity_fields();
    let copy_line = descriptor.copy_line();
    if copy_line != descriptor.presentation.copy_line {
        report.push(
            "descriptor.copy_line",
            format!("{owner} stamped copy line disagrees with the engine"),
        );
    }
    let mut surfaces_seen = BTreeSet::new();
    for projection in descriptor.project_all() {
        surfaces_seen.insert(projection.surface);
        if projection.identity != identity {
            report.push(
                "descriptor.projection_identity",
                format!(
                    "{owner} projection for {} must carry identical identity fields",
                    projection.surface.as_str()
                ),
            );
        }
        if projection.copy_line != copy_line {
            report.push(
                "descriptor.projection_copy_line",
                format!(
                    "{owner} projection for {} must carry the shared copy line",
                    projection.surface.as_str()
                ),
            );
        }
        if projection.badge.trim().is_empty()
            || projection.headline.trim().is_empty()
            || projection.detail.trim().is_empty()
        {
            report.push(
                "descriptor.projection_prose",
                format!(
                    "{owner} projection for {} must carry badge, headline, and detail",
                    projection.surface.as_str()
                ),
            );
        }
    }
    for required in SurfaceKind::ALL {
        if !surfaces_seen.contains(&required) {
            report.push(
                "descriptor.projection_coverage",
                format!("{owner} must project onto surface {}", required.as_str()),
            );
        }
    }
}

fn validate_surface_bindings(
    report: &mut ValidationReport,
    packet: &GeneratedArtifactDescriptorPacket,
) {
    let mut surfaces = BTreeSet::new();
    for surface_binding in &packet.surface_bindings {
        surfaces.insert(surface_binding.surface);
        if surface_binding.ingested_packet_id != packet.packet_id {
            report.push(
                "binding.packet_id",
                format!(
                    "binding for {} must ingest the packet id",
                    surface_binding.surface.as_str()
                ),
            );
        }
        if surface_binding.preserved_identity_fields != IDENTITY_FIELD_NAMES.to_vec() {
            report.push(
                "binding.preserved_identity_fields",
                format!(
                    "binding for {} must preserve every identity field",
                    surface_binding.surface.as_str()
                ),
            );
        }
        if surface_binding.consumer_ref.trim().is_empty()
            || surface_binding.summary.trim().is_empty()
        {
            report.push(
                "binding.prose",
                format!(
                    "binding for {} must carry a consumer ref and summary",
                    surface_binding.surface.as_str()
                ),
            );
        }
    }
    for required in SurfaceKind::ALL {
        if !surfaces.contains(&required) {
            report.push(
                "packet.binding_coverage",
                format!("packet must bind surface {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in descriptor fixture against the frozen contract.
pub fn validate_generated_artifact_descriptor_fixture(
    fixture: &GeneratedArtifactDescriptorFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }
    if fixture.fixture_id.trim().is_empty() {
        report.push("fixture.id", "fixture must carry a stable id");
    }
    if fixture.scenario.trim().is_empty() {
        report.push(
            "fixture.scenario",
            format!("fixture {} must carry a scenario label", fixture.fixture_id),
        );
    }
    if fixture.consumer_ref.trim().is_empty() {
        report.push(
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        report.push(
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    validate_descriptor(&mut report, &fixture.descriptor);

    let presentation = &fixture.descriptor.presentation;
    if fixture.expected_presented_authority != presentation.presented_authority {
        report.push(
            "fixture.expected_presented_authority",
            format!(
                "fixture {} expected presented authority disagrees with the descriptor",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_ordinary_source_claim_allowed != presentation.ordinary_source_claim_allowed
    {
        report.push(
            "fixture.expected_ordinary_source_claim_allowed",
            format!(
                "fixture {} expected ordinary-source claim disagrees with the descriptor",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_effective_edit_posture != presentation.effective_edit_posture {
        report.push(
            "fixture.expected_effective_edit_posture",
            format!(
                "fixture {} expected edit posture disagrees with the descriptor",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_block_reason_tokens != presentation.block_reason_tokens {
        report.push(
            "fixture.expected_block_reason_tokens",
            format!(
                "fixture {} expected block-reason tokens disagree with the descriptor",
                fixture.fixture_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests;
