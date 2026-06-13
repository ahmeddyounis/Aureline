//! Shared document-identity disclosures for file-bearing shell surfaces.
//!
//! The shell already projects low-level filesystem truth through
//! [`crate::path_truth`]. This module lifts that vocabulary into the
//! user-facing disclosure rows M5-class document surfaces need:
//! root class, presentation path, logical identity, canonical target,
//! alias posture, save landing class, write posture, and stable
//! labels such as `virtual`, `generated`, `archive`, or
//! `provider_backed_transient`.
//!
//! The disclosure is intentionally narrow. It does not replace the
//! underlying VFS packet, save-review sheet, or surface-specific
//! support export. It gives those consumers one controlled record they
//! can embed so notebook, retained preview, review/archive, support,
//! docs/help, and headless inspectors do not invent separate wording
//! for the same path/save-target truth.

use aureline_vfs::capabilities::RootClass;
use serde::{Deserialize, Serialize};

use crate::path_truth::SaveTargetReviewRecord;

pub mod report;

pub use report::{
    seeded_document_identity_report, DocumentIdentityReport, DocumentIdentityReportFinding,
    DocumentIdentityReportRow, DocumentIdentitySupportExport,
};

/// Stable record-kind tag for [`DocumentIdentityDisclosure`].
pub const DOCUMENT_IDENTITY_DISCLOSURE_RECORD_KIND: &str = "document_identity_disclosure_record";

/// Schema version for [`DocumentIdentityDisclosure`].
pub const DOCUMENT_IDENTITY_DISCLOSURE_SCHEMA_VERSION: u32 = 1;

/// Surface family this disclosure belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentFamilyClass {
    /// Canonical notebook file opened in the editor.
    NotebookDocument,
    /// Retained notebook output or notebook-hosted preview row.
    NotebookPreviewOutput,
    /// Authored/effective/live structured-config preview surface.
    StructuredConfigPreview,
    /// Review or support artifact opened from an archive-like carrier.
    ReviewArtifact,
    /// Provider-backed draft or transient object that is not yet durable.
    ProviderDraft,
    /// Export artifact produced from another source of truth.
    ExportArtifact,
}

impl DocumentFamilyClass {
    /// Returns the stable token for this document family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookDocument => "notebook_document",
            Self::NotebookPreviewOutput => "notebook_preview_output",
            Self::StructuredConfigPreview => "structured_config_preview",
            Self::ReviewArtifact => "review_artifact",
            Self::ProviderDraft => "provider_draft",
            Self::ExportArtifact => "export_artifact",
        }
    }
}

/// Coarse root class disclosed to users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootClassDisclosure {
    /// Local durable filesystem root.
    Local,
    /// Remote agent or managed remote mount.
    Remote,
    /// Container-backed mount.
    Container,
    /// Virtual projection with no direct durable file target.
    Virtual,
    /// Generated object or materialized derivative.
    Generated,
    /// Archive-like or sealed artifact view.
    Archive,
}

impl RootClassDisclosure {
    /// Returns the stable token for this root class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::Virtual => "virtual",
            Self::Generated => "generated",
            Self::Archive => "archive",
        }
    }

    /// Maps a VFS root class onto the disclosure vocabulary.
    pub const fn from_vfs_root_class(root_class: RootClass) -> Self {
        match root_class {
            RootClass::LocalPosixLike | RootClass::LocalWindowsLike => Self::Local,
            RootClass::RemoteAgentMount => Self::Remote,
            RootClass::ContainerMount => Self::Container,
            RootClass::VirtualGeneratedDocument => Self::Virtual,
            RootClass::ArchiveLikeView => Self::Archive,
        }
    }
}

/// Stable label classes carried by document rows and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentLabelClass {
    /// Durable local file with a direct canonical target.
    DurableLocalFile,
    /// Durable remote file that writes through a remote authority.
    RemoteFile,
    /// Virtual projection that is not itself the durable source.
    VirtualDocument,
    /// Generated or derived document.
    GeneratedDocument,
    /// Archive-backed or sealed artifact view.
    ArchiveDocument,
    /// Provider-owned transient or draft state.
    ProviderBackedTransient,
    /// Overlay or effective/live projection layered over another source.
    OverlayProjection,
    /// Export artifact derived from another source of truth.
    ExportArtifact,
}

impl DocumentLabelClass {
    /// Returns the stable token for this label class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableLocalFile => "durable_local_file",
            Self::RemoteFile => "remote_file",
            Self::VirtualDocument => "virtual_document",
            Self::GeneratedDocument => "generated_document",
            Self::ArchiveDocument => "archive_document",
            Self::ProviderBackedTransient => "provider_backed_transient",
            Self::OverlayProjection => "overlay_projection",
            Self::ExportArtifact => "export_artifact",
        }
    }
}

/// Alias posture the surface must disclose before save or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AliasStatusClass {
    /// Presentation path matches the canonical target directly.
    Direct,
    /// Presentation path is canonical but the object has other known aliases.
    CanonicalWithKnownAliases,
    /// Presentation path reaches the canonical target through a symlink alias.
    ViaSymlink,
    /// Presentation path reaches the canonical target through a case-only variant.
    ViaCaseVariant,
    /// Presentation path is a virtual or overlay projection.
    Projection,
    /// Presentation path is provider-owned or provider-remapped.
    ProviderAlias,
    /// Presentation path is an archive-inner or sealed-artifact projection.
    ArchiveProjection,
    /// Presentation and canonical targets differ but the path class is degraded.
    RedirectUnexplained,
}

impl AliasStatusClass {
    /// Returns the stable token for this alias status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::CanonicalWithKnownAliases => "canonical_with_known_aliases",
            Self::ViaSymlink => "via_symlink",
            Self::ViaCaseVariant => "via_case_variant",
            Self::Projection => "projection",
            Self::ProviderAlias => "provider_alias",
            Self::ArchiveProjection => "archive_projection",
            Self::RedirectUnexplained => "redirect_unexplained",
        }
    }
}

/// Where the next durable write would land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveTargetClass {
    /// The next save lands on a local durable file.
    LocalFile,
    /// The next save lands on a remote durable file.
    RemoteFile,
    /// The next save lands on a container-backed file.
    ContainerFile,
    /// The current object must be exported or materialized as a generated target.
    GeneratedTarget,
    /// The current object is only a virtual projection.
    VirtualProjection,
    /// The current object is an archive-backed artifact view.
    ArchiveArtifact,
    /// The next durable action produces an export artifact.
    ExportArtifact,
}

impl SaveTargetClass {
    /// Returns the stable token for this save-target class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFile => "local_file",
            Self::RemoteFile => "remote_file",
            Self::ContainerFile => "container_file",
            Self::GeneratedTarget => "generated_target",
            Self::VirtualProjection => "virtual_projection",
            Self::ArchiveArtifact => "archive_artifact",
            Self::ExportArtifact => "export_artifact",
        }
    }
}

/// Current write posture shown to the user before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritePostureClass {
    /// The current object is the editable canonical source.
    EditableCanonicalSource,
    /// Writes are allowed but guarded by a remote or revision token.
    ConditionalWrite,
    /// Save is possible only after an explicit review step.
    SaveReviewRequired,
    /// The current object must be exported before it becomes durable.
    ExportBeforeWrite,
    /// The current object must be promoted from a draft/transient state.
    PromoteBeforeSave,
    /// The current target is read-only.
    ReadOnly,
    /// The current target is inspect-only.
    InspectOnly,
}

impl WritePostureClass {
    /// Returns the stable token for this write posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditableCanonicalSource => "editable_canonical_source",
            Self::ConditionalWrite => "conditional_write",
            Self::SaveReviewRequired => "save_review_required",
            Self::ExportBeforeWrite => "export_before_write",
            Self::PromoteBeforeSave => "promote_before_save",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
        }
    }
}

/// Shared disclosure row for path/save-target truth on one document surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentIdentityDisclosure {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Document family this disclosure belongs to.
    pub document_family: DocumentFamilyClass,
    /// Stable token for [`Self::document_family`].
    pub document_family_token: String,
    /// Coarse root class disclosed to the user.
    pub root_class: RootClassDisclosure,
    /// Stable token for [`Self::root_class`].
    pub root_class_token: String,
    /// Stable label classes attached to this surface.
    pub document_labels: Vec<DocumentLabelClass>,
    /// Stable tokens for [`Self::document_labels`].
    pub document_label_tokens: Vec<String>,
    /// Presentation path or URI shown in chrome.
    pub presentation_path: String,
    /// Logical identity the workspace uses for history/review/support.
    pub logical_identity_ref: String,
    /// Canonical durable target or canonical object ref.
    pub canonical_target: String,
    /// Optional disclosure line when the canonical target differs materially.
    pub canonical_target_hint: Option<String>,
    /// Alias posture for the current open.
    pub alias_status: AliasStatusClass,
    /// Stable token for [`Self::alias_status`].
    pub alias_status_token: String,
    /// Short user-facing alias note.
    pub alias_status_label: String,
    /// Durable landing class for the next save-like action.
    pub save_target_class: SaveTargetClass,
    /// Stable token for [`Self::save_target_class`].
    pub save_target_class_token: String,
    /// User-facing save-target disclosure.
    pub save_target_label: String,
    /// Current write posture.
    pub write_posture: WritePostureClass,
    /// Stable token for [`Self::write_posture`].
    pub write_posture_token: String,
    /// User-facing write-posture label.
    pub write_posture_label: String,
    /// Human label for the current backing source.
    pub backing_source_label: String,
    /// Human label for live/cached/generated freshness.
    pub freshness_label: String,
    /// Docs/help anchor that explains this disclosure.
    pub docs_help_ref: String,
}

impl DocumentIdentityDisclosure {
    /// Normalizes derived token fields after deserialization.
    pub fn normalized(mut self) -> Self {
        self.record_kind = DOCUMENT_IDENTITY_DISCLOSURE_RECORD_KIND.to_owned();
        self.schema_version = DOCUMENT_IDENTITY_DISCLOSURE_SCHEMA_VERSION;
        self.document_family_token = self.document_family.as_str().to_owned();
        self.root_class_token = self.root_class.as_str().to_owned();
        self.document_label_tokens = self
            .document_labels
            .iter()
            .map(|label| label.as_str().to_owned())
            .collect();
        self.alias_status_token = self.alias_status.as_str().to_owned();
        self.save_target_class_token = self.save_target_class.as_str().to_owned();
        self.write_posture_token = self.write_posture.as_str().to_owned();
        self
    }

    /// Builds a disclosure from a VFS save-target review plus surface-local labels.
    pub fn from_vfs_save_target_review(
        document_family: DocumentFamilyClass,
        root_class: RootClass,
        document_labels: Vec<DocumentLabelClass>,
        review: &SaveTargetReviewRecord,
        backing_source_label: impl Into<String>,
        freshness_label: impl Into<String>,
        docs_help_ref: impl Into<String>,
    ) -> Self {
        let root_class = RootClassDisclosure::from_vfs_root_class(root_class);
        let alias_status = alias_status_from_review(review, &document_labels);
        let save_target_class = save_target_class_from_root(root_class, &document_labels);
        let write_posture = write_posture_from_review(root_class, review, &document_labels);
        let canonical_target_hint = if review.presentation_uri != review.canonical_uri {
            Some(format!("writes to {}", review.writes_to_canonical_uri))
        } else {
            None
        };

        Self {
            record_kind: String::new(),
            schema_version: 0,
            document_family,
            document_family_token: String::new(),
            root_class,
            root_class_token: String::new(),
            document_labels,
            document_label_tokens: Vec::new(),
            presentation_path: review.presentation_uri.clone(),
            logical_identity_ref: review.logical_uri.clone(),
            canonical_target: review.canonical_uri.clone(),
            canonical_target_hint,
            alias_status,
            alias_status_token: String::new(),
            alias_status_label: alias_status_label(alias_status, review),
            save_target_class,
            save_target_class_token: String::new(),
            save_target_label: save_target_label(save_target_class, review),
            write_posture,
            write_posture_token: String::new(),
            write_posture_label: write_posture_label(write_posture).to_owned(),
            backing_source_label: backing_source_label.into(),
            freshness_label: freshness_label.into(),
            docs_help_ref: docs_help_ref.into(),
        }
        .normalized()
    }

    /// Returns any required fields that are still missing.
    pub fn missing_fields(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();

        if self.presentation_path.trim().is_empty() {
            missing.push("presentation_path");
        }
        if self.logical_identity_ref.trim().is_empty() {
            missing.push("logical_identity_ref");
        }
        if self.canonical_target.trim().is_empty() {
            missing.push("canonical_target");
        }
        if self.alias_status_label.trim().is_empty() {
            missing.push("alias_status_label");
        }
        if self.save_target_label.trim().is_empty() {
            missing.push("save_target_label");
        }
        if self.write_posture_label.trim().is_empty() {
            missing.push("write_posture_label");
        }
        if self.backing_source_label.trim().is_empty() {
            missing.push("backing_source_label");
        }
        if self.freshness_label.trim().is_empty() {
            missing.push("freshness_label");
        }
        if self.docs_help_ref.trim().is_empty() {
            missing.push("docs_help_ref");
        }
        if self.document_labels.is_empty() {
            missing.push("document_labels");
        }

        missing
    }

    /// Returns the stable tokens support/export surfaces should preserve.
    pub fn identity_tokens(&self) -> Vec<String> {
        let mut tokens = vec![
            format!("family:{}", self.document_family.as_str()),
            format!("root:{}", self.root_class.as_str()),
            format!("alias:{}", self.alias_status.as_str()),
            format!("save_target:{}", self.save_target_class.as_str()),
            format!("write_posture:{}", self.write_posture.as_str()),
        ];
        tokens.extend(
            self.document_labels
                .iter()
                .map(|label| format!("label:{}", label.as_str())),
        );
        tokens
    }

    /// Renders deterministic plaintext lines for support/docs/headless consumers.
    pub fn render_plaintext_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "identity family={} root={} labels=[{}]",
            self.document_family.as_str(),
            self.root_class.as_str(),
            self.document_label_tokens.join(",")
        ));
        lines.push(format!(
            "presentation={} logical={} canonical={} alias={}",
            self.presentation_path,
            self.logical_identity_ref,
            self.canonical_target,
            self.alias_status.as_str()
        ));
        if let Some(hint) = &self.canonical_target_hint {
            lines.push(format!("canonical_hint={hint}"));
        }
        lines.push(format!(
            "save_target={} write_posture={} backing_source={} freshness={} docs_help_ref={}",
            self.save_target_class.as_str(),
            self.write_posture.as_str(),
            self.backing_source_label,
            self.freshness_label,
            self.docs_help_ref
        ));
        lines.push(format!(
            "save_target_label={} | write_posture_label={} | alias_note={}",
            self.save_target_label, self.write_posture_label, self.alias_status_label
        ));
        lines
    }
}

fn alias_status_from_review(
    review: &SaveTargetReviewRecord,
    labels: &[DocumentLabelClass],
) -> AliasStatusClass {
    if labels.contains(&DocumentLabelClass::ArchiveDocument) {
        return AliasStatusClass::ArchiveProjection;
    }
    if labels.contains(&DocumentLabelClass::ProviderBackedTransient) {
        return AliasStatusClass::ProviderAlias;
    }
    if labels.contains(&DocumentLabelClass::OverlayProjection)
        || labels.contains(&DocumentLabelClass::VirtualDocument)
    {
        return AliasStatusClass::Projection;
    }

    match review.path_truth_class.as_str() {
        "direct" => AliasStatusClass::Direct,
        "direct_with_known_aliases" => AliasStatusClass::CanonicalWithKnownAliases,
        "via_symlink" => AliasStatusClass::ViaSymlink,
        "via_case_only_variant" => AliasStatusClass::ViaCaseVariant,
        "via_remote_alias" => AliasStatusClass::ProviderAlias,
        "via_archive_inner_alias" => AliasStatusClass::ArchiveProjection,
        "divergent_unknown" => AliasStatusClass::RedirectUnexplained,
        _ => {
            if review.save_redirects_target {
                AliasStatusClass::RedirectUnexplained
            } else {
                AliasStatusClass::Direct
            }
        }
    }
}

fn alias_status_label(status: AliasStatusClass, review: &SaveTargetReviewRecord) -> String {
    match status {
        AliasStatusClass::Direct => "Presentation path is already canonical.".to_owned(),
        AliasStatusClass::CanonicalWithKnownAliases => {
            "Presentation path is canonical; alternate aliases are still known.".to_owned()
        }
        AliasStatusClass::ViaSymlink => {
            "Same canonical target opened through a symlink alias.".to_owned()
        }
        AliasStatusClass::ViaCaseVariant => {
            "Same canonical target opened through a case-only path variant.".to_owned()
        }
        AliasStatusClass::Projection => {
            "Presentation path is a projection over another source of truth.".to_owned()
        }
        AliasStatusClass::ProviderAlias => {
            "Presentation path is provider-backed and may resolve through a provider alias."
                .to_owned()
        }
        AliasStatusClass::ArchiveProjection => {
            "Presentation path is an archive-backed projection, not the durable inner source."
                .to_owned()
        }
        AliasStatusClass::RedirectUnexplained => format!(
            "Presentation path differs from canonical save target; review required before writing to {}.",
            review.writes_to_canonical_uri
        ),
    }
}

fn save_target_class_from_root(
    root_class: RootClassDisclosure,
    labels: &[DocumentLabelClass],
) -> SaveTargetClass {
    if labels.contains(&DocumentLabelClass::ExportArtifact) {
        return SaveTargetClass::ExportArtifact;
    }
    if labels.contains(&DocumentLabelClass::ArchiveDocument) {
        return SaveTargetClass::ArchiveArtifact;
    }
    if labels.contains(&DocumentLabelClass::GeneratedDocument) {
        return SaveTargetClass::GeneratedTarget;
    }
    if labels.contains(&DocumentLabelClass::VirtualDocument)
        || labels.contains(&DocumentLabelClass::OverlayProjection)
        || labels.contains(&DocumentLabelClass::ProviderBackedTransient)
    {
        return SaveTargetClass::VirtualProjection;
    }

    match root_class {
        RootClassDisclosure::Local => SaveTargetClass::LocalFile,
        RootClassDisclosure::Remote => SaveTargetClass::RemoteFile,
        RootClassDisclosure::Container => SaveTargetClass::ContainerFile,
        RootClassDisclosure::Virtual => SaveTargetClass::VirtualProjection,
        RootClassDisclosure::Generated => SaveTargetClass::GeneratedTarget,
        RootClassDisclosure::Archive => SaveTargetClass::ArchiveArtifact,
    }
}

fn save_target_label(
    save_target_class: SaveTargetClass,
    review: &SaveTargetReviewRecord,
) -> String {
    match save_target_class {
        SaveTargetClass::LocalFile => format!("Next durable write lands on local file {}.", review.writes_to_canonical_uri),
        SaveTargetClass::RemoteFile => format!("Next durable write lands on remote file {}.", review.writes_to_canonical_uri),
        SaveTargetClass::ContainerFile => format!("Next durable write lands on container file {}.", review.writes_to_canonical_uri),
        SaveTargetClass::GeneratedTarget => {
            "Current surface is generated; export or regenerate before treating it as a durable file."
                .to_owned()
        }
        SaveTargetClass::VirtualProjection => {
            "Current surface is a virtual projection; promote or export before durable write."
                .to_owned()
        }
        SaveTargetClass::ArchiveArtifact => {
            "Current surface is archive-backed; extract or reopen the backing artifact for durable edits."
                .to_owned()
        }
        SaveTargetClass::ExportArtifact => {
            "Durable output is an export artifact produced from another source of truth.".to_owned()
        }
    }
}

fn write_posture_from_review(
    root_class: RootClassDisclosure,
    review: &SaveTargetReviewRecord,
    labels: &[DocumentLabelClass],
) -> WritePostureClass {
    if labels.contains(&DocumentLabelClass::ArchiveDocument) {
        return WritePostureClass::InspectOnly;
    }
    if labels.contains(&DocumentLabelClass::ExportArtifact)
        || labels.contains(&DocumentLabelClass::GeneratedDocument)
    {
        return WritePostureClass::ExportBeforeWrite;
    }
    if labels.contains(&DocumentLabelClass::ProviderBackedTransient) {
        return WritePostureClass::PromoteBeforeSave;
    }
    if labels.contains(&DocumentLabelClass::VirtualDocument)
        || labels.contains(&DocumentLabelClass::OverlayProjection)
    {
        return WritePostureClass::InspectOnly;
    }
    if review
        .blockers
        .iter()
        .any(|blocker| blocker == "read_only" || blocker == "not_writable_per_snapshot")
    {
        return WritePostureClass::ReadOnly;
    }
    if review.review_required_before_save || review.review_required_before_rename {
        return WritePostureClass::SaveReviewRequired;
    }
    if root_class == RootClassDisclosure::Remote
        || review.atomic_write_mode == "conditional_remote_write"
    {
        return WritePostureClass::ConditionalWrite;
    }
    WritePostureClass::EditableCanonicalSource
}

fn write_posture_label(posture: WritePostureClass) -> &'static str {
    match posture {
        WritePostureClass::EditableCanonicalSource => "Editable canonical source",
        WritePostureClass::ConditionalWrite => "Conditional write with target revision checks",
        WritePostureClass::SaveReviewRequired => "Review required before save",
        WritePostureClass::ExportBeforeWrite => "Export before durable write",
        WritePostureClass::PromoteBeforeSave => "Promote draft before save",
        WritePostureClass::ReadOnly => "Read-only target",
        WritePostureClass::InspectOnly => "Inspect only",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_vfs::capabilities::RootClass;

    fn sample_disclosure() -> DocumentIdentityDisclosure {
        DocumentIdentityDisclosure {
            record_kind: String::new(),
            schema_version: 0,
            document_family: DocumentFamilyClass::NotebookDocument,
            document_family_token: String::new(),
            root_class: RootClassDisclosure::Local,
            root_class_token: String::new(),
            document_labels: vec![DocumentLabelClass::DurableLocalFile],
            document_label_tokens: Vec::new(),
            presentation_path: "workspace://repo/notebooks/report.ipynb".to_owned(),
            logical_identity_ref: "logical:notebook:report".to_owned(),
            canonical_target: "vfs:canonical:notebook:report.ipynb".to_owned(),
            canonical_target_hint: None,
            alias_status: AliasStatusClass::Direct,
            alias_status_token: String::new(),
            alias_status_label: "Presentation path is already canonical.".to_owned(),
            save_target_class: SaveTargetClass::LocalFile,
            save_target_class_token: String::new(),
            save_target_label: "Next durable write lands on local file report.ipynb.".to_owned(),
            write_posture: WritePostureClass::EditableCanonicalSource,
            write_posture_token: String::new(),
            write_posture_label: "Editable canonical source".to_owned(),
            backing_source_label: "Local notebook file".to_owned(),
            freshness_label: "Current source".to_owned(),
            docs_help_ref: "help:notebook:identity".to_owned(),
        }
        .normalized()
    }

    #[test]
    fn normalization_refreshes_tokens() {
        let disclosure = sample_disclosure();
        assert_eq!(
            disclosure.record_kind,
            DOCUMENT_IDENTITY_DISCLOSURE_RECORD_KIND
        );
        assert_eq!(
            disclosure.schema_version,
            DOCUMENT_IDENTITY_DISCLOSURE_SCHEMA_VERSION
        );
        assert_eq!(disclosure.document_family_token, "notebook_document");
        assert_eq!(disclosure.root_class_token, "local");
        assert_eq!(disclosure.document_label_tokens, vec!["durable_local_file"]);
        assert_eq!(disclosure.alias_status_token, "direct");
        assert_eq!(disclosure.save_target_class_token, "local_file");
        assert_eq!(disclosure.write_posture_token, "editable_canonical_source");
    }

    #[test]
    fn missing_fields_lists_required_gaps() {
        let disclosure = DocumentIdentityDisclosure {
            record_kind: String::new(),
            schema_version: 0,
            document_family: DocumentFamilyClass::NotebookPreviewOutput,
            document_family_token: String::new(),
            root_class: RootClassDisclosure::Generated,
            root_class_token: String::new(),
            document_labels: Vec::new(),
            document_label_tokens: Vec::new(),
            presentation_path: String::new(),
            logical_identity_ref: String::new(),
            canonical_target: String::new(),
            canonical_target_hint: None,
            alias_status: AliasStatusClass::Projection,
            alias_status_token: String::new(),
            alias_status_label: String::new(),
            save_target_class: SaveTargetClass::GeneratedTarget,
            save_target_class_token: String::new(),
            save_target_label: String::new(),
            write_posture: WritePostureClass::ExportBeforeWrite,
            write_posture_token: String::new(),
            write_posture_label: String::new(),
            backing_source_label: String::new(),
            freshness_label: String::new(),
            docs_help_ref: String::new(),
        }
        .normalized();
        let missing = disclosure.missing_fields();
        assert!(missing.contains(&"presentation_path"));
        assert!(missing.contains(&"document_labels"));
        assert!(missing.contains(&"docs_help_ref"));
    }

    #[test]
    fn vfs_mapping_keeps_virtual_and_export_truth_visible() {
        let review = SaveTargetReviewRecord {
            record_kind: "save_target_review_record".to_owned(),
            schema_version: 1,
            presentation_uri: "generated://ws/root/output.html".to_owned(),
            canonical_uri: "generated://ws/root/output.html".to_owned(),
            logical_uri: "aureline-ws://ws/root/__generated__/output.html".to_owned(),
            display_label: "output.html".to_owned(),
            root_badge: "virtual".to_owned(),
            trust_state: "trusted".to_owned(),
            atomic_write_mode: "blocked".to_owned(),
            writes_to_canonical_uri: "generated://ws/root/output.html".to_owned(),
            opens_via_alias_kind: None,
            path_truth_class: "direct".to_owned(),
            permission_summary: crate::path_truth::PermissionSummaryRecord {
                writable: false,
                mode: "0444".to_owned(),
                owner: None,
                group: None,
            },
            pinned_generation_token_kind: "content_hash".to_owned(),
            pinned_generation_token_value: "hash".to_owned(),
            review_required_before_save: false,
            review_required_before_rename: false,
            save_redirects_target: false,
            blockers: vec!["read_only".to_owned()],
            explainers: Vec::new(),
            detail_target: "aureline.workspace.showAliasDetails".to_owned(),
        };

        let disclosure = DocumentIdentityDisclosure::from_vfs_save_target_review(
            DocumentFamilyClass::ExportArtifact,
            RootClass::VirtualGeneratedDocument,
            vec![
                DocumentLabelClass::GeneratedDocument,
                DocumentLabelClass::ExportArtifact,
            ],
            &review,
            "Generated preview output",
            "Cached snapshot",
            "help:preview:generated_identity",
        );

        assert_eq!(disclosure.root_class, RootClassDisclosure::Virtual);
        assert_eq!(
            disclosure.save_target_class,
            SaveTargetClass::ExportArtifact
        );
        assert_eq!(
            disclosure.write_posture,
            WritePostureClass::ExportBeforeWrite
        );
        assert!(disclosure
            .identity_tokens()
            .iter()
            .any(|token| token == "label:generated_document"));
    }
}
