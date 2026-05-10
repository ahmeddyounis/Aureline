//! Path-truth, alias inspector, and save-target review projections.
//!
//! These projections turn the pure
//! [`aureline_vfs::identity`] records into serializable records the
//! shell can render, log, and replay against fixtures. They are
//! the protected-row consumer surface for filesystem identity:
//!
//! 1. [`PathTruthChipRecord`] is the chip the title/context bar,
//!    explorer, quick open, and search shell render next to a file
//!    label.
//! 2. [`AliasInspectorRecord`] is the alias inspector the support
//!    surface shows when a user clicks the chip.
//! 3. [`SaveTargetReviewRecord`] is the pre-write review surface
//!    that explains where bytes will land before a save commits;
//!    it complements the post-conflict
//!    [`crate::save_review::SaveReviewSheetRecord`] surface.
//!
//! Materialization is fixture-driven via the
//! `fixtures/vfs/path_truth_cases/*.json` set, where each case
//! pairs a synthetic-root scenario with the expected projections.

use aureline_vfs::identity::{
    AliasInspectionEntry, AliasInspectionRecord as VfsAliasInspectionRecord, AliasKind,
    PathTruthChip, PermissionSummary, SaveTargetReviewRecord as VfsSaveTargetReviewRecord,
    derive_path_truth_chip, inspect_aliases, review_save_target,
};
use aureline_vfs::{IdentityRecord, SaveTargetToken, TrustState};
use serde::{Deserialize, Serialize};

/// Path-truth chip record — projection of [`PathTruthChip`] plus
/// shell-side schema metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathTruthChipRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub class: String,
    pub presentation_uri: String,
    pub canonical_uri: String,
    pub logical_uri: String,
    pub root_badge: String,
    pub display_label: String,
    pub trust_state: String,
    pub opens_via_alias_kind: Option<String>,
    pub alias_count: u32,
    pub save_redirects_target: bool,
    pub detail_target: String,
    pub summary: String,
}

/// One alias-inspector row, serializable variant of
/// [`AliasInspectionEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasInspectorEntryRecord {
    pub alias_uri: String,
    pub alias_kind: String,
    pub resolution_chain: Vec<String>,
    pub is_canonical: bool,
    pub is_presentation: bool,
}

/// Alias-inspector record. Mirrors
/// [`aureline_vfs::AliasInspectionRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasInspectorRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub presentation_uri: String,
    pub canonical_uri: String,
    pub logical_uri: String,
    pub display_label: String,
    pub root_badge: String,
    pub entries: Vec<AliasInspectorEntryRecord>,
    pub distinct_alias_kinds: Vec<String>,
    pub presentation_alias_missing: bool,
}

/// Permission-snapshot summary record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionSummaryRecord {
    pub writable: bool,
    pub mode: String,
    pub owner: Option<String>,
    pub group: Option<String>,
}

/// Pre-write save-target review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveTargetReviewRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub presentation_uri: String,
    pub canonical_uri: String,
    pub logical_uri: String,
    pub display_label: String,
    pub root_badge: String,
    pub trust_state: String,
    pub atomic_write_mode: String,
    pub writes_to_canonical_uri: String,
    pub opens_via_alias_kind: Option<String>,
    pub path_truth_class: String,
    pub permission_summary: PermissionSummaryRecord,
    pub pinned_generation_token_kind: String,
    pub pinned_generation_token_value: String,
    pub review_required_before_save: bool,
    pub review_required_before_rename: bool,
    pub save_redirects_target: bool,
    pub blockers: Vec<String>,
    pub explainers: Vec<String>,
    pub detail_target: String,
}

/// Combined record projecting all three surfaces from one open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathTruthProjection {
    pub record_kind: String,
    pub schema_version: u32,
    pub chip: PathTruthChipRecord,
    pub alias_inspector: AliasInspectorRecord,
    pub save_target_review: SaveTargetReviewRecord,
}

const PATH_TRUTH_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Materialize the path-truth chip record from an
/// [`IdentityRecord`].
pub fn materialize_path_truth_chip_record(record: &IdentityRecord) -> PathTruthChipRecord {
    let chip = derive_path_truth_chip(record);
    chip_record_from(&chip)
}

/// Materialize the alias-inspector record from an
/// [`IdentityRecord`].
pub fn materialize_alias_inspector_record(record: &IdentityRecord) -> AliasInspectorRecord {
    let inspection = inspect_aliases(record);
    alias_inspector_record_from(&inspection)
}

/// Materialize the pre-write save-target review record from a
/// [`SaveTargetToken`].
pub fn materialize_save_target_review_record(token: &SaveTargetToken) -> SaveTargetReviewRecord {
    let review = review_save_target(token);
    save_target_review_record_from(&review)
}

/// Materialize all three projections from a [`SaveTargetToken`].
pub fn materialize_path_truth_projection(token: &SaveTargetToken) -> PathTruthProjection {
    PathTruthProjection {
        record_kind: "path_truth_projection_record".to_string(),
        schema_version: PATH_TRUTH_PROJECTION_SCHEMA_VERSION,
        chip: chip_record_from(&derive_path_truth_chip(&token.identity)),
        alias_inspector: alias_inspector_record_from(&inspect_aliases(&token.identity)),
        save_target_review: save_target_review_record_from(&review_save_target(token)),
    }
}

fn chip_record_from(chip: &PathTruthChip) -> PathTruthChipRecord {
    PathTruthChipRecord {
        record_kind: "path_truth_chip_record".to_string(),
        schema_version: PATH_TRUTH_PROJECTION_SCHEMA_VERSION,
        class: chip.class.as_str().to_string(),
        presentation_uri: chip.presentation_uri.as_str().to_string(),
        canonical_uri: chip.canonical_uri.as_str().to_string(),
        logical_uri: chip.logical_uri.as_str().to_string(),
        root_badge: chip.root_badge.clone(),
        display_label: chip.display_label.clone(),
        trust_state: trust_state_token(chip.trust_state).to_string(),
        opens_via_alias_kind: chip.opens_via_alias_kind.map(alias_kind_token),
        alias_count: chip.alias_count as u32,
        save_redirects_target: chip.save_redirects_target,
        detail_target: chip.detail_target.to_string(),
        summary: chip.summary.clone(),
    }
}

fn alias_inspector_record_from(record: &VfsAliasInspectionRecord) -> AliasInspectorRecord {
    AliasInspectorRecord {
        record_kind: "alias_inspector_record".to_string(),
        schema_version: PATH_TRUTH_PROJECTION_SCHEMA_VERSION,
        presentation_uri: record.presentation_uri.as_str().to_string(),
        canonical_uri: record.canonical_uri.as_str().to_string(),
        logical_uri: record.logical_uri.as_str().to_string(),
        display_label: record.display_label.clone(),
        root_badge: record.root_badge.clone(),
        entries: record.entries.iter().map(alias_inspector_entry_from).collect(),
        distinct_alias_kinds: record
            .distinct_alias_kinds
            .iter()
            .map(|kind| alias_kind_token(*kind))
            .collect(),
        presentation_alias_missing: record.presentation_alias_missing,
    }
}

fn alias_inspector_entry_from(entry: &AliasInspectionEntry) -> AliasInspectorEntryRecord {
    AliasInspectorEntryRecord {
        alias_uri: entry.alias_uri.as_str().to_string(),
        alias_kind: alias_kind_token(entry.alias_kind),
        resolution_chain: entry.resolution_chain.clone(),
        is_canonical: entry.is_canonical,
        is_presentation: entry.is_presentation,
    }
}

fn save_target_review_record_from(review: &VfsSaveTargetReviewRecord) -> SaveTargetReviewRecord {
    SaveTargetReviewRecord {
        record_kind: "save_target_review_record".to_string(),
        schema_version: PATH_TRUTH_PROJECTION_SCHEMA_VERSION,
        presentation_uri: review.presentation_uri.as_str().to_string(),
        canonical_uri: review.canonical_uri.as_str().to_string(),
        logical_uri: review.logical_uri.as_str().to_string(),
        display_label: review.display_label.clone(),
        root_badge: review.root_badge.clone(),
        trust_state: trust_state_token(review.trust_state).to_string(),
        atomic_write_mode: review.atomic_write_mode.as_str().to_string(),
        writes_to_canonical_uri: review.writes_to_canonical_uri.as_str().to_string(),
        opens_via_alias_kind: review.opens_via_alias_kind.map(alias_kind_token),
        path_truth_class: review.path_truth_class.as_str().to_string(),
        permission_summary: permission_summary_from(&review.permission_summary),
        pinned_generation_token_kind: review.pinned_generation_token_kind.as_str().to_string(),
        pinned_generation_token_value: review.pinned_generation_token_value.clone(),
        review_required_before_save: review.review_required_before_save,
        review_required_before_rename: review.review_required_before_rename,
        save_redirects_target: review.save_redirects_target,
        blockers: review
            .blockers
            .iter()
            .map(|blocker| blocker.as_str().to_string())
            .collect(),
        explainers: review.explainers.clone(),
        detail_target: review.detail_target.clone(),
    }
}

fn permission_summary_from(summary: &PermissionSummary) -> PermissionSummaryRecord {
    PermissionSummaryRecord {
        writable: summary.writable,
        mode: summary.mode.clone(),
        owner: summary.owner.clone(),
        group: summary.group.clone(),
    }
}

fn trust_state_token(trust_state: TrustState) -> &'static str {
    trust_state.as_str()
}

fn alias_kind_token(kind: AliasKind) -> String {
    kind.as_str().to_string()
}

/// Render the path-truth chip in a single short line.
pub fn path_truth_chip_lines(record: &PathTruthChipRecord) -> Vec<String> {
    vec![
        format!(
            "[{badge}] {label} — {class}",
            badge = record.root_badge,
            label = record.display_label,
            class = record.class
        ),
        record.summary.clone(),
        format!(
            "presentation: {pres}\ncanonical:    {canon}\nlogical:      {logical}",
            pres = record.presentation_uri,
            canon = record.canonical_uri,
            logical = record.logical_uri
        ),
    ]
}

/// Render the alias-inspector body lines for the support surface.
pub fn alias_inspector_lines(record: &AliasInspectorRecord) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Alias inspector — {label} ({badge})",
        label = record.display_label,
        badge = record.root_badge
    ));
    lines.push(format!("presentation: {}", record.presentation_uri));
    lines.push(format!("canonical:    {}", record.canonical_uri));
    lines.push(format!("logical:      {}", record.logical_uri));

    if record.entries.is_empty() {
        lines.push("(no aliases recorded)".to_string());
    } else {
        lines.push(format!("{} alias entr{}:", record.entries.len(), if record.entries.len() == 1 { "y" } else { "ies" }));
        for entry in &record.entries {
            let mut markers: Vec<&str> = Vec::new();
            if entry.is_presentation {
                markers.push("presentation");
            }
            if entry.is_canonical {
                markers.push("canonical");
            }
            let marker = if markers.is_empty() {
                String::new()
            } else {
                format!(" [{}]", markers.join(", "))
            };
            lines.push(format!(
                "  {kind}{marker}: {uri}",
                kind = entry.alias_kind,
                marker = marker,
                uri = entry.alias_uri,
            ));
            for step in &entry.resolution_chain {
                lines.push(format!("    {step}"));
            }
        }
    }

    if record.presentation_alias_missing {
        lines.push(
            "WARNING: presentation URI is not represented in the alias set; review before saving."
                .to_string(),
        );
    }

    lines
}

/// Render the pre-write save-target review body lines.
pub fn save_target_review_lines(record: &SaveTargetReviewRecord) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Save target review — {label}",
        label = record.display_label
    ));
    lines.push(format!(
        "writes to: {target} (mode={mode})",
        target = record.writes_to_canonical_uri,
        mode = record.atomic_write_mode,
    ));
    lines.push(format!(
        "presentation: {pres}\ncanonical:    {canon}\nlogical:      {logical}",
        pres = record.presentation_uri,
        canon = record.canonical_uri,
        logical = record.logical_uri
    ));
    lines.push(format!(
        "trust: {trust}   path-truth: {class}   alias_via: {via}",
        trust = record.trust_state,
        class = record.path_truth_class,
        via = record
            .opens_via_alias_kind
            .clone()
            .unwrap_or_else(|| "(none)".to_string())
    ));
    lines.push(format!(
        "permission: writable={w} mode={m} owner={o} group={g}",
        w = record.permission_summary.writable,
        m = record.permission_summary.mode,
        o = record.permission_summary.owner.clone().unwrap_or_else(|| "?".to_string()),
        g = record.permission_summary.group.clone().unwrap_or_else(|| "?".to_string())
    ));
    lines.push(format!(
        "compare-before-write: kind={k} value={v}",
        k = record.pinned_generation_token_kind,
        v = record.pinned_generation_token_value
    ));
    if record.blockers.is_empty() {
        lines.push("blockers: (none)".to_string());
    } else {
        lines.push(format!("blockers: {}", record.blockers.join(", ")));
    }
    if !record.explainers.is_empty() {
        lines.push("explainers:".to_string());
        for line in &record.explainers {
            lines.push(format!("  {line}"));
        }
    }
    lines
}

fn sanitize_filename(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

/// Persist a path-truth projection to `.logs/path_truth/`. Used by
/// the support / save-review surfaces when they want a replayable
/// record without taking a hard dependency on telemetry.
pub fn write_path_truth_projection_log(record: &PathTruthProjection, label: &str) {
    let root = std::path::PathBuf::from(".logs").join("path_truth");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }
    let filename = format!(
        "{}.path_truth_projection.json",
        sanitize_filename(label),
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_vfs::save::open_save_target;
    use aureline_vfs::{
        Alias, AliasKind, CapabilityFlags, CaseSensitivity, HookCounters, NormalizationForm,
        PermissionSnapshot, RootClass, SymlinkEscapePolicy, SyntheticRootBuilder,
        TrustState as VfsTrustState, VfsUri,
    };
    use serde::Deserialize;
    use std::path::Path;

    #[derive(Debug, Clone, Deserialize)]
    struct PathTruthFixture {
        #[serde(default)]
        case_id: String,
        #[serde(default)]
        title: String,
        input: PathTruthFixtureInput,
        expected: PathTruthProjection,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct PathTruthFixtureInput {
        presentation_uri: String,
        canonical_uri: String,
        logical_uri: String,
        display_label: String,
        root_badge: String,
        root_class: String,
        trust_state: String,
        normalization_form: String,
        strongest_token_base: String,
        initial_generation: u64,
        permission_writable: bool,
        permission_mode: String,
        permission_owner: Option<String>,
        permission_group: Option<String>,
        review_required_before_save: bool,
        review_required_before_rename: bool,
        read_only: bool,
        policy_constrained: bool,
        case_sensitivity: String,
        symlink_escape_policy: String,
        observed_at: String,
        aliases: Vec<PathTruthFixtureAlias>,
        presentation_alias_kind: Option<String>,
        presentation_resolution_chain: Vec<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct PathTruthFixtureAlias {
        alias_uri: String,
        alias_kind: String,
        resolution_chain: Vec<String>,
    }

    fn parse_root_class(value: &str) -> RootClass {
        match value {
            "local_posix_like" => RootClass::LocalPosixLike,
            "local_windows_like" => RootClass::LocalWindowsLike,
            "remote_agent_mount" => RootClass::RemoteAgentMount,
            "container_mount" => RootClass::ContainerMount,
            "virtual_generated_document" => RootClass::VirtualGeneratedDocument,
            "archive_like_view" => RootClass::ArchiveLikeView,
            other => panic!("unsupported root_class in fixture: {other}"),
        }
    }

    fn parse_trust_state(value: &str) -> VfsTrustState {
        match value {
            "trusted" => VfsTrustState::Trusted,
            "restricted" => VfsTrustState::Restricted,
            "pending_evaluation" => VfsTrustState::PendingEvaluation,
            other => panic!("unsupported trust_state in fixture: {other}"),
        }
    }

    fn parse_normalization_form(value: &str) -> NormalizationForm {
        match value {
            "none" => NormalizationForm::None,
            "nfc" => NormalizationForm::Nfc,
            "nfd" => NormalizationForm::Nfd,
            "mixed_observed" => NormalizationForm::MixedObserved,
            other => panic!("unsupported normalization_form: {other}"),
        }
    }

    fn parse_case_sensitivity(value: &str) -> CaseSensitivity {
        match value {
            "sensitive" => CaseSensitivity::Sensitive,
            "insensitive_preserving" => CaseSensitivity::InsensitivePreserving,
            "insensitive_non_preserving" => CaseSensitivity::InsensitiveNonPreserving,
            other => panic!("unsupported case_sensitivity: {other}"),
        }
    }

    fn parse_symlink_escape_policy(value: &str) -> SymlinkEscapePolicy {
        match value {
            "allow" => SymlinkEscapePolicy::Allow,
            "warn" => SymlinkEscapePolicy::Warn,
            "block" => SymlinkEscapePolicy::Block,
            other => panic!("unsupported symlink_escape_policy: {other}"),
        }
    }

    fn parse_alias_kind(value: &str) -> AliasKind {
        match value {
            "symlink" => AliasKind::Symlink,
            "junction" => AliasKind::Junction,
            "hardlink_sibling" => AliasKind::HardlinkSibling,
            "case_only_variant" => AliasKind::CaseOnlyVariant,
            "unicode_normalization_variant" => AliasKind::UnicodeNormalizationVariant,
            "remote_alias" => AliasKind::RemoteAlias,
            "bind_mount_alias" => AliasKind::BindMountAlias,
            "container_mount_alias" => AliasKind::ContainerMountAlias,
            "archive_inner_alias" => AliasKind::ArchiveInnerAlias,
            other => panic!("unsupported alias_kind: {other}"),
        }
    }

    fn build_capability_flags(input: &PathTruthFixtureInput) -> CapabilityFlags {
        CapabilityFlags {
            supports_atomic_replace: !input.read_only,
            supports_in_place_write: !input.read_only,
            supports_conditional_remote_write: false,
            case_sensitivity: parse_case_sensitivity(&input.case_sensitivity),
            unicode_normalization: parse_normalization_form(&input.normalization_form),
            supports_case_only_rename: true,
            supports_unicode_normalization_rename: true,
            symlink_escape_policy: parse_symlink_escape_policy(&input.symlink_escape_policy),
            read_only: input.read_only,
            policy_constrained: input.policy_constrained,
            review_required_before_save: input.review_required_before_save,
            review_required_before_rename: input.review_required_before_rename,
            remote_container_adaptation: false,
        }
    }

    fn build_token(input: &PathTruthFixtureInput) -> SaveTargetToken {
        let aliases: Vec<Alias> = input
            .aliases
            .iter()
            .map(|alias| Alias {
                alias_uri: VfsUri::parse(alias.alias_uri.clone()).unwrap(),
                alias_kind: parse_alias_kind(&alias.alias_kind),
                resolution_chain: alias.resolution_chain.clone(),
            })
            .collect();

        let mut root = SyntheticRootBuilder::new(
            "root-1",
            parse_root_class(&input.root_class),
            build_capability_flags(input),
        )
        .with_workspace_id("ws-path-truth")
        .with_trust_state(parse_trust_state(&input.trust_state))
        .add_canonical_object(
            input.canonical_uri.clone(),
            input.logical_uri.clone(),
            parse_normalization_form(&input.normalization_form),
            input.strongest_token_base.clone(),
            input.initial_generation,
            vec![],
            PermissionSnapshot {
                writable: input.permission_writable,
                mode: input.permission_mode.clone(),
                owner: input.permission_owner.clone(),
                group: input.permission_group.clone(),
                acl_summary: None,
            },
            aliases,
            b"hello".to_vec(),
        )
        .add_presentation(
            input.presentation_uri.clone(),
            input.display_label.clone(),
            input.canonical_uri.clone(),
            input.presentation_alias_kind.as_deref().map(parse_alias_kind),
            input.presentation_resolution_chain.clone(),
        );

        if input.presentation_uri != input.canonical_uri {
            root = root.add_presentation(
                input.canonical_uri.clone(),
                input.display_label.clone(),
                input.canonical_uri.clone(),
                None,
                vec!["-> canonical".to_owned()],
            );
        }

        let root = root.build();
        let presentation_uri = VfsUri::parse(input.presentation_uri.clone()).unwrap();
        let mut counters = HookCounters::default();
        let mut token =
            open_save_target(&root, &presentation_uri, input.observed_at.clone(), &mut counters)
                .expect("open_save_target must succeed for path_truth fixture");
        token.identity.presentation_path.root_badge = input.root_badge.clone();
        token
    }

    #[test]
    fn materialize_path_truth_projection_matches_fixtures() {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vfs/path_truth_cases");

        let mut count = 0usize;
        for entry in std::fs::read_dir(&root_dir).expect("path_truth_cases directory must exist") {
            let entry = entry.expect("path_truth_cases directory entry must read");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            let payload = std::fs::read_to_string(&path).expect("path_truth fixture must read");
            let fixture: PathTruthFixture =
                serde_json::from_str(&payload).expect("path_truth fixture must parse");

            let token = build_token(&fixture.input);
            let projection = materialize_path_truth_projection(&token);

            assert_eq!(
                projection,
                fixture.expected,
                "path_truth projection mismatch for case {} ({}): {}",
                fixture.case_id,
                fixture.title,
                path.display()
            );
            count += 1;
        }

        assert!(
            count >= 3,
            "expected at least 3 path_truth fixtures, found {count}"
        );
    }
}
