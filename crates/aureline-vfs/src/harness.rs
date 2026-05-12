//! Smoke harness for the VFS / save prototype.
//!
//! Drives the prototype through a frozen scenario table — one row
//! per ADR 0006 failure case the save pipeline must name with its
//! own vocabulary (atomic commit, case-only variant, symlink alias,
//! hardlink sibling, Unicode-normalization variant, external-change
//! conflict, review-required gate, read-only block, remote
//! conditional conflict, watcher fallback) — and emits a
//! reviewable save-plan record per scenario plus an aggregate.
//!
//! Metrics are counts only (no wall-clock timings), so the
//! committed seed under `artifacts/fs/save_plan_examples/` stays
//! byte-stable across hosts. The benchmark lab layers timing on
//! top when it scores against protected-hot-path budgets.
//!
//! The harness never touches the real filesystem. Every scenario
//! routes through a [`crate::synthetic::SyntheticRoot`] so a
//! machine's inode, mtime, or filesystem casing cannot leak into
//! the emitted seed.

use std::fmt::Write as _;

use crate::capabilities::{
    AtomicWriteMode, CapabilityFlags, CaseSensitivity, FallbackIdentityTokenKind,
    NormalizationForm, RootClass, SymlinkEscapePolicy,
};
use crate::hooks::HookCounters;
use crate::identity::{
    Alias, AliasKind, FallbackIdentityToken, IdentityRecord, PresentationPath, TrustState,
};
use crate::save::{
    attempt_save, open_save_target, PermissionSnapshot, SaveManifest, SaveOutcome, SavePlan,
    SaveRequest, SaveTargetToken,
};
use crate::synthetic::{SyntheticRoot, SyntheticRootBuilder};
use crate::uri_model::VfsUri;
use crate::watcher::{WatcherHealth, WatcherHealthFrame, WatcherRegistry, WatcherSource};

/// Frozen corpus identifier. Bumped only when the harness's output
/// schema itself changes (not on scenario additions).
pub const CORPUS_ID: &str = "aureline.vfs_save_plan_examples.v2";

/// Schema version for the emitted save-plan JSON.
pub const SCHEMA_VERSION: u32 = 2;

/// One row of the frozen scenario table.
#[derive(Debug, Clone, Copy)]
pub struct ScenarioSpec {
    pub label: &'static str,
    pub corpus_case_id: &'static str,
    pub related_fixture_ids: &'static [&'static str],
    pub rename_matrix_row_refs: &'static [&'static str],
    pub scenario_summary: &'static str,
    pub builder: fn() -> ScenarioFixture,
    pub expected_outcome: SaveOutcome,
}

/// Everything a scenario needs to run: the workspace state, the
/// presentation URI the editor opened, the participant failure
/// (if any), and the watcher events the harness should apply
/// before issuing the save.
#[derive(Debug, Clone)]
pub struct ScenarioFixture {
    pub root: SyntheticRoot,
    pub presentation_uri: VfsUri,
    pub watcher_script: Vec<WatcherScriptedStep>,
    pub external_change_before_save: bool,
    pub participant_failure: Option<String>,
    pub save_participant_group_id: Option<String>,
    pub checkpoint_ref: Option<String>,
    pub new_content: Vec<u8>,
    pub reviewer_prologue: Vec<String>,
}

/// One step in the watcher script: how the watcher for the root
/// transitioned between open and save-commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatcherScriptedStep {
    pub root_id: String,
    pub source: WatcherSource,
    pub health: WatcherHealth,
    pub observed_at: String,
    pub reason_code: Option<String>,
}

/// Report for one scenario. Wraps the [`SavePlan`] the pipeline
/// produced and the hook-counter snapshot taken at end of run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReport {
    pub label: &'static str,
    pub corpus_case_id: &'static str,
    pub related_fixture_ids: Vec<&'static str>,
    pub rename_matrix_row_refs: Vec<&'static str>,
    pub root_class: RootClass,
    pub scenario_summary: &'static str,
    pub expected_outcome: SaveOutcome,
    pub plan: SavePlan,
    pub counters: HookCounters,
}

/// Aggregate across all scenarios. Counts only; wall-clock timing
/// belongs to the benchmark lab.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AggregateReport {
    pub total_scenarios: u64,
    pub total_committed: u64,
    pub total_external_change_detected: u64,
    pub total_save_conflict: u64,
    pub total_read_only_or_policy_blocked: u64,
    pub total_review_required_before_save: u64,
    pub total_save_participant_failed: u64,
    pub total_degraded_guarantee_declared: u64,
    pub total_watcher_frames_emitted: u64,
    pub total_watcher_degraded_frames: u64,
    pub total_alias_converge_hits: u64,
}

/// Full harness output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessReport {
    pub schema_version: u32,
    pub corpus_id: &'static str,
    pub scenarios: Vec<ScenarioReport>,
    pub aggregate: AggregateReport,
}

/// Run the full scenario table and produce a report. Harness is
/// deterministic: same inputs, same outputs, byte-for-byte.
pub fn run_harness() -> HarnessReport {
    let mut scenarios = Vec::with_capacity(SCENARIOS.len());
    let mut agg = AggregateReport::default();
    for spec in SCENARIOS {
        let report = run_scenario(spec);
        match report.plan.save_manifest.outcome {
            SaveOutcome::Committed => agg.total_committed += 1,
            SaveOutcome::ExternalChangeDetected => agg.total_external_change_detected += 1,
            SaveOutcome::SaveConflict => agg.total_save_conflict += 1,
            SaveOutcome::ReadOnlyOrPolicyBlocked => agg.total_read_only_or_policy_blocked += 1,
            SaveOutcome::ReviewRequiredBeforeSave => agg.total_review_required_before_save += 1,
            SaveOutcome::SaveParticipantFailed => agg.total_save_participant_failed += 1,
            _ => {}
        }
        agg.total_degraded_guarantee_declared += report.counters.vfs_degraded_guarantee_declared;
        agg.total_alias_converge_hits += report.counters.vfs_alias_converge;
        agg.total_watcher_frames_emitted += report.plan.watcher_frames.len() as u64;
        agg.total_watcher_degraded_frames += report
            .plan
            .watcher_frames
            .iter()
            .filter(|f| f.watcher_health.is_degraded())
            .count() as u64;
        scenarios.push(report);
    }
    agg.total_scenarios = SCENARIOS.len() as u64;
    HarnessReport {
        schema_version: SCHEMA_VERSION,
        corpus_id: CORPUS_ID,
        scenarios,
        aggregate: agg,
    }
}

fn run_scenario(spec: &ScenarioSpec) -> ScenarioReport {
    let ScenarioFixture {
        mut root,
        presentation_uri,
        watcher_script,
        external_change_before_save,
        participant_failure,
        save_participant_group_id,
        checkpoint_ref,
        new_content,
        reviewer_prologue,
    } = (spec.builder)();

    let root_class = root.envelope().root_class;

    let mut counters = HookCounters::default();
    counters.vfs_root_attach += 1;

    let mut watcher_registry = WatcherRegistry::new();
    let root_id = root.envelope().root_id.clone();
    let initial_source = root.envelope().watcher_source;
    watcher_registry.register(
        root_id.clone(),
        initial_source,
        "mono:0000:00:00:00.0000".to_owned(),
        Some("initial_attach".to_owned()),
    );
    counters.vfs_watcher_health_changed += 1;
    for step in watcher_script {
        let changed = watcher_registry.transition(
            &step.root_id,
            step.health,
            step.observed_at,
            step.reason_code,
        );
        if changed {
            counters.vfs_watcher_health_changed += 1;
        }
    }

    let mut reviewer_notes = reviewer_prologue;

    let token = match open_save_target(
        &root,
        &presentation_uri,
        "mono:1200:00:00:00.0100",
        &mut counters,
    ) {
        Ok(t) => t,
        Err(err) => {
            // Unknown-presentation / missing-token paths are
            // fixture bugs, not product paths; surface them as a
            // degenerate save plan so every scenario emits a
            // record even in the bug case.
            reviewer_notes.push(format!("open_save_target failed: {err}"));
            let manifest = degenerate_manifest(presentation_uri.clone(), err.to_string());
            let plan = SavePlan {
                label: spec.label.to_owned(),
                scenario_summary: spec.scenario_summary.to_owned(),
                identity_record: degenerate_identity(&presentation_uri),
                save_target_token: degenerate_token(&presentation_uri),
                save_manifest: manifest,
                watcher_frames: watcher_registry.frames().to_vec(),
                reviewer_notes,
            };
            return ScenarioReport {
                label: spec.label,
                corpus_case_id: spec.corpus_case_id,
                related_fixture_ids: spec.related_fixture_ids.to_vec(),
                rename_matrix_row_refs: spec.rename_matrix_row_refs.to_vec(),
                root_class,
                scenario_summary: spec.scenario_summary,
                expected_outcome: spec.expected_outcome,
                plan,
                counters,
            };
        }
    };

    reviewer_notes.push(format!(
        "selected save mode: {} (preferred by envelope)",
        token.atomic_write_mode.as_str()
    ));
    for frame in watcher_registry.frames() {
        reviewer_notes.push(format!(
            "watcher[{}]: source={} health={} reason={}",
            frame.root_id,
            frame.watcher_source.as_str(),
            frame.watcher_health.as_str(),
            frame.reason_code.as_deref().unwrap_or(""),
        ));
    }

    if external_change_before_save {
        let canonical_uri = token
            .identity
            .canonical_filesystem_object
            .canonical_uri
            .clone();
        if let Some(new_gen) = root.apply_external_change(canonical_uri.as_str()) {
            counters.vfs_watcher_event += 1;
            reviewer_notes.push(format!(
                "external sibling writer bumped generation to {new_gen} on {canonical_uri}"
            ));
        }
    }

    let identity_for_plan = token.identity.clone();
    let token_for_plan = token.clone();

    let request = SaveRequest {
        token,
        new_content,
        save_participant_group_id,
        checkpoint_ref,
        committed_at: "mono:1200:00:00:00.5000".to_owned(),
        participant_failure,
    };

    let manifest = attempt_save(&mut root, request, &mut counters);
    reviewer_notes.push(outcome_note(&manifest));

    let plan = SavePlan {
        label: spec.label.to_owned(),
        scenario_summary: spec.scenario_summary.to_owned(),
        identity_record: identity_for_plan,
        save_target_token: token_for_plan,
        save_manifest: manifest,
        watcher_frames: watcher_registry.frames().to_vec(),
        reviewer_notes,
    };

    ScenarioReport {
        label: spec.label,
        corpus_case_id: spec.corpus_case_id,
        related_fixture_ids: spec.related_fixture_ids.to_vec(),
        rename_matrix_row_refs: spec.rename_matrix_row_refs.to_vec(),
        root_class,
        scenario_summary: spec.scenario_summary,
        expected_outcome: spec.expected_outcome,
        plan,
        counters,
    }
}

fn outcome_note(manifest: &SaveManifest) -> String {
    let detail = manifest.failure_detail.as_deref().unwrap_or("");
    format!(
        "outcome: {} | capability_mode: {} | detail: {}",
        manifest.outcome.as_str(),
        manifest.capability_mode.as_str(),
        detail,
    )
}

fn degenerate_manifest(presentation_uri: VfsUri, detail: String) -> SaveManifest {
    use crate::identity::{CanonicalFilesystemObject, IdentityToken};
    use crate::save::{GenerationToken, GenerationTokenKind};
    SaveManifest {
        presentation_path: PresentationPath {
            uri: presentation_uri.clone(),
            display_label: String::new(),
            root_badge: String::new(),
        },
        canonical_filesystem_object: CanonicalFilesystemObject {
            canonical_uri: presentation_uri,
            normalization_form: NormalizationForm::None,
            strongest_identity_token: IdentityToken {
                kind: crate::capabilities::StrongestIdentityTokenKind::ContentHashOnly,
                value: String::new(),
            },
            fallback_identity_tokens: vec![],
        },
        generation_token: GenerationToken {
            kind: GenerationTokenKind::ContentHash,
            value: String::new(),
        },
        capability_mode: AtomicWriteMode::Blocked,
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: String::new(),
        outcome: SaveOutcome::WrongTargetPrevented,
        failure_detail: Some(detail),
    }
}

fn degenerate_identity(presentation_uri: &VfsUri) -> IdentityRecord {
    use crate::identity::{
        AliasSet, CanonicalFilesystemObject, IdentityToken, LogicalWorkspaceIdentity,
    };
    let placeholder_logical =
        VfsUri::workspace_logical_uri("ws-invalid", "root-invalid", "missing")
            .expect("degenerate logical uri must be valid");
    IdentityRecord {
        presentation_path: PresentationPath {
            uri: presentation_uri.clone(),
            display_label: String::new(),
            root_badge: String::new(),
        },
        logical_workspace_identity: LogicalWorkspaceIdentity {
            workspace_id: String::new(),
            root_id: String::new(),
            logical_uri: placeholder_logical,
            trust_state: TrustState::PendingEvaluation,
            policy_scope: None,
        },
        canonical_filesystem_object: CanonicalFilesystemObject {
            canonical_uri: presentation_uri.clone(),
            normalization_form: NormalizationForm::None,
            strongest_identity_token: IdentityToken {
                kind: crate::capabilities::StrongestIdentityTokenKind::ContentHashOnly,
                value: String::new(),
            },
            fallback_identity_tokens: vec![],
        },
        alias_set: AliasSet::default(),
    }
}

fn degenerate_token(presentation_uri: &VfsUri) -> SaveTargetToken {
    use crate::save::{CompareBeforeWriteGenerationToken, GenerationTokenKind};
    SaveTargetToken {
        identity: degenerate_identity(presentation_uri),
        capability_flags: degenerate_flags(),
        atomic_write_mode: AtomicWriteMode::Blocked,
        compare_before_write_generation_token: CompareBeforeWriteGenerationToken {
            kind: GenerationTokenKind::ContentHash,
            value: String::new(),
            observed_at: String::new(),
        },
        permission_snapshot: PermissionSnapshot::read_only_default(),
        review_required_before_save: true,
        review_required_before_rename: true,
    }
}

fn degenerate_flags() -> CapabilityFlags {
    CapabilityFlags {
        supports_atomic_replace: false,
        supports_in_place_write: false,
        supports_conditional_remote_write: false,
        case_sensitivity: CaseSensitivity::Sensitive,
        unicode_normalization: NormalizationForm::None,
        supports_case_only_rename: false,
        supports_unicode_normalization_rename: false,
        symlink_escape_policy: SymlinkEscapePolicy::Block,
        read_only: true,
        policy_constrained: true,
        review_required_before_save: true,
        review_required_before_rename: true,
        remote_container_adaptation: false,
    }
}

// ---------------------------------------------------------------------------
// Scenario table. One row per failure case the ADR names.
// ---------------------------------------------------------------------------

pub const SCENARIOS: &[ScenarioSpec] = &[
    ScenarioSpec {
        label: "local_atomic_save_happy_path",
        corpus_case_id: "corpus.fs.identity.local_atomic_save_happy_path",
        related_fixture_ids: &[],
        rename_matrix_row_refs: &[],
        scenario_summary:
            "Local POSIX-like root; atomic_replace preferred; compare-before-write holds; commits.",
        builder: scenarios::local_atomic_save_happy_path,
        expected_outcome: SaveOutcome::Committed,
    },
    ScenarioSpec {
        label: "case_only_difference",
        corpus_case_id: "corpus.fs.identity.case_only_difference",
        related_fixture_ids: &["corpus.fs.alias.case_fold_collision_insensitive_root"],
        rename_matrix_row_refs: &[
            "rename_matrix.local_apfs_insensitive_preview_required",
            "rename_matrix.local_windows_insensitive_preview_required",
        ],
        scenario_summary:
            "Presentation uri differs from canonical by case only; alias convergence fires; atomic commit proceeds.",
        builder: scenarios::case_only_difference,
        expected_outcome: SaveOutcome::Committed,
    },
    ScenarioSpec {
        label: "symlink_alias",
        corpus_case_id: "corpus.fs.identity.symlink_alias",
        related_fixture_ids: &[
            "corpus.fs.alias.presentation_vs_canonical_symlink",
            "corpus.fs.alias.symlink_escape_review",
        ],
        rename_matrix_row_refs: &[],
        scenario_summary:
            "Presentation uri is a symlink into the workspace; resolution chain disclosed; atomic commit proceeds on canonical.",
        builder: scenarios::symlink_alias,
        expected_outcome: SaveOutcome::Committed,
    },
    ScenarioSpec {
        label: "hardlink_sibling",
        corpus_case_id: "corpus.fs.identity.hardlink_sibling",
        related_fixture_ids: &["corpus.fs.alias.hardlink_sibling_shared_authority"],
        rename_matrix_row_refs: &[],
        scenario_summary:
            "Canonical object has a hardlink sibling; alias disclosed; atomic commit on canonical warns the reviewer that the sibling receives the same bytes.",
        builder: scenarios::hardlink_sibling,
        expected_outcome: SaveOutcome::Committed,
    },
    ScenarioSpec {
        label: "unicode_normalization_variant",
        corpus_case_id: "corpus.fs.identity.unicode_normalization_variant",
        related_fixture_ids: &["corpus.fs.alias.unicode_normalization_variant"],
        rename_matrix_row_refs: &["rename_matrix.local_apfs_unicode_preview_required"],
        scenario_summary:
            "Presentation uri uses NFD; canonical uses NFC; normalization-variant alias disclosed; atomic commit proceeds on canonical.",
        builder: scenarios::unicode_normalization_variant,
        expected_outcome: SaveOutcome::Committed,
    },
    ScenarioSpec {
        label: "external_change_detected",
        corpus_case_id: "corpus.fs.identity.external_change_detected",
        related_fixture_ids: &[],
        rename_matrix_row_refs: &[],
        scenario_summary:
            "Sibling writer bumped the generation between open and save; compare-before-write catches the mismatch and blocks the commit.",
        builder: scenarios::external_change_detected,
        expected_outcome: SaveOutcome::ExternalChangeDetected,
    },
    ScenarioSpec {
        label: "review_required_before_save",
        corpus_case_id: "corpus.fs.identity.review_required_before_save",
        related_fixture_ids: &[],
        rename_matrix_row_refs: &[],
        scenario_summary:
            "Policy-scoped workspace advertises review_required_before_save; pipeline halts before any bytes move.",
        builder: scenarios::review_required_before_save,
        expected_outcome: SaveOutcome::ReviewRequiredBeforeSave,
    },
    ScenarioSpec {
        label: "read_only_root_blocked",
        corpus_case_id: "corpus.fs.identity.read_only_root_blocked",
        related_fixture_ids: &[],
        rename_matrix_row_refs: &["rename_matrix.archive_like_unsupported"],
        scenario_summary:
            "Read-only overlay (archive-like view); select_save_mode returns blocked; save fails closed with read_only_or_policy_blocked.",
        builder: scenarios::read_only_root_blocked,
        expected_outcome: SaveOutcome::ReadOnlyOrPolicyBlocked,
    },
    ScenarioSpec {
        label: "remote_conditional_conflict",
        corpus_case_id: "corpus.fs.identity.remote_conditional_conflict",
        related_fixture_ids: &["corpus.fs.alias.remote_alias_degraded_root"],
        rename_matrix_row_refs: &[
            "rename_matrix.remote_agent_attested_degraded",
            "rename_matrix.remote_agent_unattested_unsupported",
        ],
        scenario_summary:
            "Remote-agent root; conditional_remote_write preferred; sibling writer bumped the revision token; pipeline returns save_conflict.",
        builder: scenarios::remote_conditional_conflict,
        expected_outcome: SaveOutcome::SaveConflict,
    },
    ScenarioSpec {
        label: "watcher_degradation",
        corpus_case_id: "corpus.fs.identity.watcher_degradation",
        related_fixture_ids: &[],
        rename_matrix_row_refs: &[],
        scenario_summary:
            "Local root's OS watcher drops into fallback_polling mid-session; watcher-health frames emitted; compare-before-write is still the correctness floor and the commit proceeds.",
        builder: scenarios::watcher_degradation,
        expected_outcome: SaveOutcome::Committed,
    },
    ScenarioSpec {
        label: "save_participant_failed",
        corpus_case_id: "corpus.fs.identity.save_participant_failed",
        related_fixture_ids: &[],
        rename_matrix_row_refs: &[],
        scenario_summary:
            "Save participant (text-normalisation) raises; pipeline records save_participant_failed and never runs compare-before-write.",
        builder: scenarios::save_participant_failed,
        expected_outcome: SaveOutcome::SaveParticipantFailed,
    },
];

// ---------------------------------------------------------------------------
// Scenario builders. Each returns a fresh synthetic root and the
// presentation URI the editor "opened" for the scenario.
// ---------------------------------------------------------------------------

mod scenarios {
    use super::*;

    fn uri(raw: &str) -> VfsUri {
        VfsUri::parse(raw.to_owned()).expect("scenario uri must be valid")
    }

    pub(super) fn posix_flags() -> CapabilityFlags {
        CapabilityFlags {
            supports_atomic_replace: true,
            supports_in_place_write: true,
            supports_conditional_remote_write: false,
            case_sensitivity: CaseSensitivity::InsensitivePreserving,
            unicode_normalization: NormalizationForm::MixedObserved,
            supports_case_only_rename: true,
            supports_unicode_normalization_rename: true,
            symlink_escape_policy: SymlinkEscapePolicy::Warn,
            read_only: false,
            policy_constrained: false,
            review_required_before_save: false,
            review_required_before_rename: false,
            remote_container_adaptation: false,
        }
    }

    fn posix_fallback_tokens() -> Vec<FallbackIdentityToken> {
        vec![FallbackIdentityToken {
            kind: FallbackIdentityTokenKind::InodeMtimeSize,
            value: "ino:101/mtime:1000/size:40".to_owned(),
        }]
    }

    pub(super) fn local_atomic_save_happy_path() -> ScenarioFixture {
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .with_mount_graph_hash("mount-graph:abc123")
            .add_canonical_object(
                "file:///ws/README.md",
                "aureline-ws://ws-aureline-primary/root-1/README.md",
                NormalizationForm::Nfc,
                "dev:1/ino:101",
                4,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                vec![],
                b"# project\n".to_vec(),
            )
            .add_presentation(
                "file:///ws/README.md",
                "README.md",
                "file:///ws/README.md",
                None,
                vec!["presentation -> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri("file:///ws/README.md"),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-1".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: Some("spg:trailing_whitespace+final_newline".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"# project\n\nhello world\n".to_vec(),
            reviewer_prologue: vec![
                "root_class=local_posix_like; strongest=device_inode_generation".to_owned(),
                "preferred save_mode=atomic_replace; no alias convergence expected".to_owned(),
            ],
        }
    }

    pub(super) fn case_only_difference() -> ScenarioFixture {
        let canonical = "file:///ws/src/Lib.rs";
        let presentation = "file:///ws/src/lib.rs";
        let aliases = vec![Alias {
            alias_uri: uri(presentation),
            alias_kind: AliasKind::CaseOnlyVariant,
            resolution_chain: vec![
                format!("opened: {presentation}"),
                "case-folded on insensitive_preserving root".to_owned(),
                format!("canonical: {canonical}"),
            ],
        }];
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-1/src/Lib.rs",
                NormalizationForm::Nfc,
                "dev:1/ino:202",
                7,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                aliases.clone(),
                b"pub fn greet() {}\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "Lib.rs",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .add_presentation(
                presentation,
                "lib.rs",
                canonical,
                Some(AliasKind::CaseOnlyVariant),
                aliases[0].resolution_chain.clone(),
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(presentation),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-1".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: Some("spg:trailing_whitespace".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"pub fn greet() { println!(\"hi\"); }\n".to_vec(),
            reviewer_prologue: vec![
                "root advertises case_sensitivity=insensitive_preserving".to_owned(),
                "presentation 'lib.rs' case-folds to canonical 'Lib.rs'".to_owned(),
            ],
        }
    }

    pub(super) fn symlink_alias() -> ScenarioFixture {
        let canonical = "file:///ws/pkg/core/config.toml";
        let presentation = "file:///ws/config.toml";
        let aliases = vec![Alias {
            alias_uri: uri(presentation),
            alias_kind: AliasKind::Symlink,
            resolution_chain: vec![
                format!("opened: {presentation}"),
                "symlink -> pkg/core/config.toml".to_owned(),
                format!("canonical: {canonical}"),
            ],
        }];
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-1/pkg/core/config.toml",
                NormalizationForm::Nfc,
                "dev:1/ino:303",
                2,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                aliases.clone(),
                b"[package]\nname=\"core\"\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "config.toml",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .add_presentation(
                presentation,
                "config.toml",
                canonical,
                Some(AliasKind::Symlink),
                aliases[0].resolution_chain.clone(),
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(presentation),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-1".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: Some("spg:toml_format".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"[package]\nname=\"core\"\nversion=\"0.0.0\"\n".to_vec(),
            reviewer_prologue: vec![
                "presentation is a symlink; canonical write target is under pkg/core/".to_owned(),
                "symlink_escape_policy=warn; symlink stays inside root so no escape".to_owned(),
            ],
        }
    }

    pub(super) fn hardlink_sibling() -> ScenarioFixture {
        let canonical = "file:///ws/bin/tool.sh";
        let sibling = "file:///ws/bin/tool-latest";
        let aliases = vec![Alias {
            alias_uri: uri(sibling),
            alias_kind: AliasKind::HardlinkSibling,
            resolution_chain: vec![
                format!("opened canonical: {canonical}"),
                format!("hardlink sibling observed: {sibling}"),
                "same inode; writes via either path affect the other".to_owned(),
            ],
        }];
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-1/bin/tool.sh",
                NormalizationForm::Nfc,
                "dev:1/ino:404",
                5,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                aliases.clone(),
                b"#!/bin/sh\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "tool.sh",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(canonical),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-1".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: Some("spg:script_format".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"#!/bin/sh\necho hi\n".to_vec(),
            reviewer_prologue: vec![
                "canonical has one hardlink sibling; save surface MUST disclose that the sibling receives the new bytes".to_owned(),
            ],
        }
    }

    pub(super) fn unicode_normalization_variant() -> ScenarioFixture {
        // NFC vs NFD. We do not carry real unicode here to keep the
        // fixture reviewable in ASCII diff tools; the alias_kind
        // and resolution_chain carry the story.
        let canonical = "file:///ws/notes/cafe-nfc.md";
        let presentation = "file:///ws/notes/cafe-nfd.md";
        let aliases = vec![Alias {
            alias_uri: uri(presentation),
            alias_kind: AliasKind::UnicodeNormalizationVariant,
            resolution_chain: vec![
                format!("opened: {presentation} (nfd)"),
                "normalization folded nfd -> nfc".to_owned(),
                format!("canonical: {canonical} (nfc)"),
            ],
        }];
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-1/notes/cafe.md",
                NormalizationForm::Nfc,
                "dev:1/ino:505",
                1,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                aliases.clone(),
                b"cafe notes\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "cafe-nfc.md",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .add_presentation(
                presentation,
                "cafe-nfd.md",
                canonical,
                Some(AliasKind::UnicodeNormalizationVariant),
                aliases[0].resolution_chain.clone(),
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(presentation),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-1".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: Some("spg:markdown_format".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"cafe notes\nupdated\n".to_vec(),
            reviewer_prologue: vec![
                "root advertises unicode_normalization=mixed_observed".to_owned(),
                "presentation (nfd) resolves to canonical (nfc); no case-only variant".to_owned(),
            ],
        }
    }

    pub(super) fn external_change_detected() -> ScenarioFixture {
        let canonical = "file:///ws/src/main.rs";
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-1/src/main.rs",
                NormalizationForm::Nfc,
                "dev:1/ino:606",
                10,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                vec![],
                b"fn main() {}\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "main.rs",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(canonical),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-1".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: true,
            participant_failure: None,
            save_participant_group_id: Some("spg:rustfmt".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"fn main() { println!(\"editor\"); }\n".to_vec(),
            reviewer_prologue: vec![
                "sibling writer (e.g. git or second editor) bumped the generation after open"
                    .to_owned(),
                "compare-before-write is the correctness floor; watcher is a latency optimisation"
                    .to_owned(),
            ],
        }
    }

    pub(super) fn review_required_before_save() -> ScenarioFixture {
        let mut flags = posix_flags();
        flags.review_required_before_save = true;
        flags.review_required_before_rename = true;
        flags.policy_constrained = false;
        let canonical = "file:///ws/infra/prod.tf";
        let root = SyntheticRootBuilder::new("root-infra", RootClass::LocalPosixLike, flags)
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-infra/infra/prod.tf",
                NormalizationForm::Nfc,
                "dev:1/ino:707",
                3,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                vec![],
                b"resource {}\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "prod.tf",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(canonical),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-infra".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: Some("spg:tf_format".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"resource {}\n# edited\n".to_vec(),
            reviewer_prologue: vec![
                "workspace policy attaches review_required_before_save to this root".to_owned(),
                "save must route through review workflow; no bytes move from pipeline".to_owned(),
            ],
        }
    }

    pub(super) fn read_only_root_blocked() -> ScenarioFixture {
        let flags = CapabilityFlags {
            supports_atomic_replace: false,
            supports_in_place_write: false,
            supports_conditional_remote_write: false,
            case_sensitivity: CaseSensitivity::Sensitive,
            unicode_normalization: NormalizationForm::None,
            supports_case_only_rename: false,
            supports_unicode_normalization_rename: false,
            symlink_escape_policy: SymlinkEscapePolicy::Block,
            read_only: true,
            policy_constrained: false,
            review_required_before_save: false,
            review_required_before_rename: false,
            remote_container_adaptation: false,
        };
        let canonical = "archive:///deps/openssl-3.0.0/crypto/rsa/rsa_lib.c";
        let root = SyntheticRootBuilder::new("root-archive", RootClass::ArchiveLikeView, flags)
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-archive/deps/openssl-3.0.0/crypto/rsa/rsa_lib.c",
                NormalizationForm::Nfc,
                "archive:openssl-3.0.0#rsa_lib.c",
                1,
                vec![FallbackIdentityToken {
                    kind: FallbackIdentityTokenKind::ContentHash,
                    value: "sha256:0000000000000000".to_owned(),
                }],
                PermissionSnapshot::read_only_default(),
                vec![],
                b"/* openssl */\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "rsa_lib.c",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(canonical),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-archive".to_owned(),
                source: WatcherSource::PollingFallback,
                health: WatcherHealth::FallbackPolling,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("archive_view_has_no_native_watcher".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: None,
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"/* edited */\n".to_vec(),
            reviewer_prologue: vec![
                "archive-like view; read-only overlay; select_save_mode returns blocked".to_owned(),
                "surface MUST route to save-as on a writable root, not attempt in-place".to_owned(),
            ],
        }
    }

    pub(super) fn remote_conditional_conflict() -> ScenarioFixture {
        let flags = CapabilityFlags {
            supports_atomic_replace: false,
            supports_in_place_write: false,
            supports_conditional_remote_write: true,
            case_sensitivity: CaseSensitivity::Sensitive,
            unicode_normalization: NormalizationForm::None,
            supports_case_only_rename: false,
            supports_unicode_normalization_rename: false,
            symlink_escape_policy: SymlinkEscapePolicy::Block,
            read_only: false,
            policy_constrained: false,
            review_required_before_save: false,
            review_required_before_rename: false,
            remote_container_adaptation: true,
        };
        let canonical = "agent://remote-1/workspace/src/lib.rs";
        let root = SyntheticRootBuilder::new("root-remote", RootClass::RemoteAgentMount, flags)
            .with_preferred_save_mode(AtomicWriteMode::ConditionalRemoteWrite)
            .with_fallback_token_kinds(vec![FallbackIdentityTokenKind::RemoteRevisionToken])
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-remote/src/lib.rs",
                NormalizationForm::Nfc,
                "remote-agent:lib.rs#rev",
                17,
                vec![FallbackIdentityToken {
                    kind: FallbackIdentityTokenKind::RemoteRevisionToken,
                    value: "rev:17".to_owned(),
                }],
                PermissionSnapshot::writable_default(),
                vec![],
                b"// remote\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "lib.rs",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(canonical),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-remote".to_owned(),
                source: WatcherSource::RemoteAgentWatcherStream,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("remote_stream_connected".to_owned()),
            }],
            external_change_before_save: true,
            participant_failure: None,
            save_participant_group_id: Some("spg:remote_normalize".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"// remote edited\n".to_vec(),
            reviewer_prologue: vec![
                "remote-agent root; preferred save_mode=conditional_remote_write".to_owned(),
                "sibling commit on remote bumped the revision token; pipeline yields save_conflict"
                    .to_owned(),
            ],
        }
    }

    pub(super) fn watcher_degradation() -> ScenarioFixture {
        let canonical = "file:///ws/logs/app.log";
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-1/logs/app.log",
                NormalizationForm::Nfc,
                "dev:1/ino:808",
                2,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                vec![],
                b"boot\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "app.log",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(canonical),
            watcher_script: vec![
                WatcherScriptedStep {
                    root_id: "root-1".to_owned(),
                    source: WatcherSource::OsNativeWatcher,
                    health: WatcherHealth::Healthy,
                    observed_at: "mono:0000:00:00:00.0500".to_owned(),
                    reason_code: Some("watcher_primed".to_owned()),
                },
                WatcherScriptedStep {
                    root_id: "root-1".to_owned(),
                    source: WatcherSource::OsNativeWatcher,
                    health: WatcherHealth::Degraded,
                    observed_at: "mono:0000:00:00:00.2500".to_owned(),
                    reason_code: Some("os_native_buffer_overflow".to_owned()),
                },
                WatcherScriptedStep {
                    root_id: "root-1".to_owned(),
                    source: WatcherSource::OsNativeWatcher,
                    health: WatcherHealth::FallbackPolling,
                    observed_at: "mono:0000:00:00:00.3000".to_owned(),
                    reason_code: Some("falling_back_to_polling".to_owned()),
                },
            ],
            external_change_before_save: false,
            participant_failure: None,
            save_participant_group_id: Some("spg:log_rotate".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"boot\nrunning\n".to_vec(),
            reviewer_prologue: vec![
                "os watcher degraded to fallback_polling mid-session".to_owned(),
                "subscription freshness_downgrade fires with stale_reason=watcher_dropped"
                    .to_owned(),
                "compare-before-write is the correctness floor; commit still proceeds".to_owned(),
            ],
        }
    }

    pub(super) fn save_participant_failed() -> ScenarioFixture {
        let canonical = "file:///ws/src/app.rs";
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                canonical,
                "aureline-ws://ws-aureline-primary/root-1/src/app.rs",
                NormalizationForm::Nfc,
                "dev:1/ino:909",
                4,
                posix_fallback_tokens(),
                PermissionSnapshot::writable_default(),
                vec![],
                b"fn app() {}\n".to_vec(),
            )
            .add_presentation(
                canonical,
                "app.rs",
                canonical,
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        ScenarioFixture {
            root,
            presentation_uri: uri(canonical),
            watcher_script: vec![WatcherScriptedStep {
                root_id: "root-1".to_owned(),
                source: WatcherSource::OsNativeWatcher,
                health: WatcherHealth::Healthy,
                observed_at: "mono:0000:00:00:00.0500".to_owned(),
                reason_code: Some("watcher_primed".to_owned()),
            }],
            external_change_before_save: false,
            participant_failure: Some(
                "save_participant 'rustfmt' exited with non-zero status".to_owned(),
            ),
            save_participant_group_id: Some("spg:rustfmt".to_owned()),
            checkpoint_ref: Some("checkpoint:open".to_owned()),
            new_content: b"fn app() { todo!() }\n".to_vec(),
            reviewer_prologue: vec![
                "save participant fails before compare-before-write".to_owned(),
                "no bytes move; editor retains buffer; banner shows participant stderr".to_owned(),
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// JSON renderer. Hand-rolled so we do not pull in a serde dep just
// for the prototype's committed seeds. Output stays byte-stable as
// long as this renderer stays byte-stable.
// ---------------------------------------------------------------------------

/// Render the aggregate + per-scenario report as one JSON blob.
pub fn report_to_json(report: &HarnessReport) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    kv_u64(
        &mut out,
        1,
        "schema_version",
        u64::from(report.schema_version),
        false,
    );
    kv_str(&mut out, 1, "corpus_id", report.corpus_id, false);
    write_aggregate(&mut out, 1, &report.aggregate);
    key(&mut out, 1, "scenarios");
    out.push_str(" [\n");
    for (i, scenario) in report.scenarios.iter().enumerate() {
        let last = i + 1 == report.scenarios.len();
        write_scenario(&mut out, 2, scenario, last);
    }
    indent(&mut out, 1);
    out.push_str("]\n");
    out.push_str("}\n");
    out
}

/// Render a single scenario's save plan as a standalone JSON
/// document. Used by the bench binary when it emits one file per
/// scenario into `artifacts/fs/save_plan_examples/`.
pub fn scenario_to_json(report: &ScenarioReport) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    kv_u64(
        &mut out,
        1,
        "schema_version",
        u64::from(SCHEMA_VERSION),
        false,
    );
    kv_str(&mut out, 1, "corpus_id", CORPUS_ID, false);
    kv_str(&mut out, 1, "label", report.label, false);
    kv_str(&mut out, 1, "corpus_case_id", report.corpus_case_id, false);
    kv_str(&mut out, 1, "root_class", report.root_class.as_str(), false);
    write_string_slice(
        &mut out,
        1,
        "related_fixture_ids",
        &report.related_fixture_ids,
        false,
    );
    write_string_slice(
        &mut out,
        1,
        "rename_matrix_row_refs",
        &report.rename_matrix_row_refs,
        false,
    );
    kv_str(
        &mut out,
        1,
        "scenario_summary",
        report.scenario_summary,
        false,
    );
    kv_str(
        &mut out,
        1,
        "expected_outcome",
        report.expected_outcome.as_str(),
        false,
    );
    write_plan_body(&mut out, 1, &report.plan);
    write_counters(&mut out, 1, &report.counters, true);
    out.push_str("}\n");
    out
}

fn write_aggregate(out: &mut String, depth: usize, agg: &AggregateReport) {
    key(out, depth, "aggregate");
    out.push_str(" {\n");
    kv_u64(
        out,
        depth + 1,
        "total_scenarios",
        agg.total_scenarios,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_committed",
        agg.total_committed,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_external_change_detected",
        agg.total_external_change_detected,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_save_conflict",
        agg.total_save_conflict,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_read_only_or_policy_blocked",
        agg.total_read_only_or_policy_blocked,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_review_required_before_save",
        agg.total_review_required_before_save,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_save_participant_failed",
        agg.total_save_participant_failed,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_degraded_guarantee_declared",
        agg.total_degraded_guarantee_declared,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_watcher_frames_emitted",
        agg.total_watcher_frames_emitted,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_watcher_degraded_frames",
        agg.total_watcher_degraded_frames,
        false,
    );
    kv_u64(
        out,
        depth + 1,
        "total_alias_converge_hits",
        agg.total_alias_converge_hits,
        true,
    );
    indent(out, depth);
    out.push_str("},\n");
}

fn write_scenario(out: &mut String, depth: usize, s: &ScenarioReport, last: bool) {
    indent(out, depth);
    out.push_str("{\n");
    kv_str(out, depth + 1, "label", s.label, false);
    kv_str(out, depth + 1, "corpus_case_id", s.corpus_case_id, false);
    kv_str(out, depth + 1, "root_class", s.root_class.as_str(), false);
    write_string_slice(
        out,
        depth + 1,
        "related_fixture_ids",
        &s.related_fixture_ids,
        false,
    );
    write_string_slice(
        out,
        depth + 1,
        "rename_matrix_row_refs",
        &s.rename_matrix_row_refs,
        false,
    );
    kv_str(
        out,
        depth + 1,
        "scenario_summary",
        s.scenario_summary,
        false,
    );
    kv_str(
        out,
        depth + 1,
        "expected_outcome",
        s.expected_outcome.as_str(),
        false,
    );
    write_plan_body(out, depth + 1, &s.plan);
    write_counters(out, depth + 1, &s.counters, true);
    indent(out, depth);
    if last {
        out.push_str("}\n");
    } else {
        out.push_str("},\n");
    }
}

fn write_plan_body(out: &mut String, depth: usize, plan: &SavePlan) {
    write_identity(out, depth, &plan.identity_record);
    write_save_target_token(out, depth, &plan.save_target_token);
    write_save_manifest(out, depth, &plan.save_manifest);
    write_watcher_frames(out, depth, &plan.watcher_frames);
    write_reviewer_notes(out, depth, &plan.reviewer_notes);
}

fn write_identity(out: &mut String, depth: usize, id: &IdentityRecord) {
    key(out, depth, "identity_record");
    out.push_str(" {\n");
    // presentation_path
    key(out, depth + 1, "presentation_path");
    out.push_str(" {\n");
    kv_str(
        out,
        depth + 2,
        "uri",
        id.presentation_path.uri.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 2,
        "display_label",
        &id.presentation_path.display_label,
        false,
    );
    kv_str(
        out,
        depth + 2,
        "root_badge",
        &id.presentation_path.root_badge,
        true,
    );
    indent(out, depth + 1);
    out.push_str("},\n");
    // logical
    let logical = &id.logical_workspace_identity;
    key(out, depth + 1, "logical_workspace_identity");
    out.push_str(" {\n");
    kv_str(out, depth + 2, "workspace_id", &logical.workspace_id, false);
    kv_str(out, depth + 2, "root_id", &logical.root_id, false);
    kv_str(
        out,
        depth + 2,
        "logical_uri",
        logical.logical_uri.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 2,
        "trust_state",
        logical.trust_state.as_str(),
        false,
    );
    kv_str_opt(
        out,
        depth + 2,
        "policy_scope",
        logical.policy_scope.as_deref(),
        true,
    );
    indent(out, depth + 1);
    out.push_str("},\n");
    // canonical
    let canonical = &id.canonical_filesystem_object;
    key(out, depth + 1, "canonical_filesystem_object");
    out.push_str(" {\n");
    kv_str(
        out,
        depth + 2,
        "canonical_uri",
        canonical.canonical_uri.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 2,
        "normalization_form",
        canonical.normalization_form.as_str(),
        false,
    );
    key(out, depth + 2, "strongest_identity_token");
    out.push_str(" {\n");
    kv_str(
        out,
        depth + 3,
        "kind",
        canonical.strongest_identity_token.kind.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 3,
        "value",
        &canonical.strongest_identity_token.value,
        true,
    );
    indent(out, depth + 2);
    out.push_str("},\n");
    key(out, depth + 2, "fallback_identity_tokens");
    out.push_str(" [\n");
    for (i, fb) in canonical.fallback_identity_tokens.iter().enumerate() {
        let last = i + 1 == canonical.fallback_identity_tokens.len();
        indent(out, depth + 3);
        out.push_str("{\n");
        kv_str(out, depth + 4, "kind", fb.kind.as_str(), false);
        kv_str(out, depth + 4, "value", &fb.value, true);
        indent(out, depth + 3);
        if last {
            out.push_str("}\n");
        } else {
            out.push_str("},\n");
        }
    }
    indent(out, depth + 2);
    out.push_str("]\n");
    indent(out, depth + 1);
    out.push_str("},\n");
    // alias set
    key(out, depth + 1, "alias_set");
    out.push_str(" {\n");
    key(out, depth + 2, "aliases");
    out.push_str(" [\n");
    for (i, a) in id.alias_set.aliases.iter().enumerate() {
        let last = i + 1 == id.alias_set.aliases.len();
        indent(out, depth + 3);
        out.push_str("{\n");
        kv_str(out, depth + 4, "alias_uri", a.alias_uri.as_str(), false);
        kv_str(out, depth + 4, "alias_kind", a.alias_kind.as_str(), false);
        key(out, depth + 4, "resolution_chain");
        out.push_str(" [\n");
        for (j, step) in a.resolution_chain.iter().enumerate() {
            let step_last = j + 1 == a.resolution_chain.len();
            indent(out, depth + 5);
            out.push_str(&json_quote(step));
            if step_last {
                out.push('\n');
            } else {
                out.push_str(",\n");
            }
        }
        indent(out, depth + 4);
        out.push_str("]\n");
        indent(out, depth + 3);
        if last {
            out.push_str("}\n");
        } else {
            out.push_str("},\n");
        }
    }
    indent(out, depth + 2);
    out.push_str("]\n");
    indent(out, depth + 1);
    out.push_str("}\n");
    indent(out, depth);
    out.push_str("},\n");
}

fn write_save_target_token(out: &mut String, depth: usize, token: &SaveTargetToken) {
    key(out, depth, "save_target_token");
    out.push_str(" {\n");
    kv_str(
        out,
        depth + 1,
        "atomic_write_mode",
        token.atomic_write_mode.as_str(),
        false,
    );
    write_capability_flags(out, depth + 1, &token.capability_flags);
    key(out, depth + 1, "compare_before_write_generation_token");
    out.push_str(" {\n");
    kv_str(
        out,
        depth + 2,
        "kind",
        token.compare_before_write_generation_token.kind.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 2,
        "value",
        &token.compare_before_write_generation_token.value,
        false,
    );
    kv_str(
        out,
        depth + 2,
        "observed_at",
        &token.compare_before_write_generation_token.observed_at,
        true,
    );
    indent(out, depth + 1);
    out.push_str("},\n");
    key(out, depth + 1, "permission_snapshot");
    out.push_str(" {\n");
    kv_bool(
        out,
        depth + 2,
        "writable",
        token.permission_snapshot.writable,
        false,
    );
    kv_str(
        out,
        depth + 2,
        "mode",
        &token.permission_snapshot.mode,
        false,
    );
    kv_str_opt(
        out,
        depth + 2,
        "owner",
        token.permission_snapshot.owner.as_deref(),
        false,
    );
    kv_str_opt(
        out,
        depth + 2,
        "group",
        token.permission_snapshot.group.as_deref(),
        false,
    );
    kv_str_opt(
        out,
        depth + 2,
        "acl_summary",
        token.permission_snapshot.acl_summary.as_deref(),
        true,
    );
    indent(out, depth + 1);
    out.push_str("},\n");
    kv_bool(
        out,
        depth + 1,
        "review_required_before_save",
        token.review_required_before_save,
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "review_required_before_rename",
        token.review_required_before_rename,
        true,
    );
    indent(out, depth);
    out.push_str("},\n");
}

fn write_capability_flags(out: &mut String, depth: usize, flags: &CapabilityFlags) {
    key(out, depth, "capability_flags");
    out.push_str(" {\n");
    kv_bool(
        out,
        depth + 1,
        "supports_atomic_replace",
        flags.supports_atomic_replace,
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "supports_in_place_write",
        flags.supports_in_place_write,
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "supports_conditional_remote_write",
        flags.supports_conditional_remote_write,
        false,
    );
    kv_str(
        out,
        depth + 1,
        "case_sensitivity",
        flags.case_sensitivity.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 1,
        "unicode_normalization",
        flags.unicode_normalization.as_str(),
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "supports_case_only_rename",
        flags.supports_case_only_rename,
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "supports_unicode_normalization_rename",
        flags.supports_unicode_normalization_rename,
        false,
    );
    kv_str(
        out,
        depth + 1,
        "symlink_escape_policy",
        flags.symlink_escape_policy.as_str(),
        false,
    );
    kv_bool(out, depth + 1, "read_only", flags.read_only, false);
    kv_bool(
        out,
        depth + 1,
        "policy_constrained",
        flags.policy_constrained,
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "review_required_before_save",
        flags.review_required_before_save,
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "review_required_before_rename",
        flags.review_required_before_rename,
        false,
    );
    kv_bool(
        out,
        depth + 1,
        "remote_container_adaptation",
        flags.remote_container_adaptation,
        true,
    );
    indent(out, depth);
    out.push_str("},\n");
}

fn write_save_manifest(out: &mut String, depth: usize, m: &SaveManifest) {
    key(out, depth, "save_manifest");
    out.push_str(" {\n");
    kv_str(out, depth + 1, "outcome", m.outcome.as_str(), false);
    kv_str(
        out,
        depth + 1,
        "capability_mode",
        m.capability_mode.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 1,
        "presentation_uri",
        m.presentation_path.uri.as_str(),
        false,
    );
    kv_str(
        out,
        depth + 1,
        "canonical_uri",
        m.canonical_filesystem_object.canonical_uri.as_str(),
        false,
    );
    key(out, depth + 1, "generation_token");
    out.push_str(" {\n");
    kv_str(
        out,
        depth + 2,
        "kind",
        m.generation_token.kind.as_str(),
        false,
    );
    kv_str(out, depth + 2, "value", &m.generation_token.value, true);
    indent(out, depth + 1);
    out.push_str("},\n");
    kv_str_opt(
        out,
        depth + 1,
        "save_participant_group_id",
        m.save_participant_group_id.as_deref(),
        false,
    );
    kv_str_opt(
        out,
        depth + 1,
        "checkpoint_ref",
        m.checkpoint_ref.as_deref(),
        false,
    );
    kv_str(out, depth + 1, "committed_at", &m.committed_at, false);
    kv_str_opt(
        out,
        depth + 1,
        "failure_detail",
        m.failure_detail.as_deref(),
        true,
    );
    indent(out, depth);
    out.push_str("},\n");
}

fn write_watcher_frames(out: &mut String, depth: usize, frames: &[WatcherHealthFrame]) {
    key(out, depth, "watcher_frames");
    out.push_str(" [\n");
    for (i, f) in frames.iter().enumerate() {
        let last = i + 1 == frames.len();
        indent(out, depth + 1);
        out.push_str("{\n");
        kv_str(out, depth + 2, "root_id", &f.root_id, false);
        kv_str(
            out,
            depth + 2,
            "watcher_source",
            f.watcher_source.as_str(),
            false,
        );
        kv_str(
            out,
            depth + 2,
            "watcher_health",
            f.watcher_health.as_str(),
            false,
        );
        kv_str_opt(
            out,
            depth + 2,
            "reason_code",
            f.reason_code.as_deref(),
            false,
        );
        kv_str(out, depth + 2, "observed_at", &f.observed_at, true);
        indent(out, depth + 1);
        if last {
            out.push_str("}\n");
        } else {
            out.push_str("},\n");
        }
    }
    indent(out, depth);
    out.push_str("],\n");
}

fn write_reviewer_notes(out: &mut String, depth: usize, notes: &[String]) {
    key(out, depth, "reviewer_notes");
    out.push_str(" [\n");
    for (i, note) in notes.iter().enumerate() {
        let last = i + 1 == notes.len();
        indent(out, depth + 1);
        out.push_str(&json_quote(note));
        if last {
            out.push('\n');
        } else {
            out.push_str(",\n");
        }
    }
    indent(out, depth);
    out.push_str("],\n");
}

fn write_counters(out: &mut String, depth: usize, counters: &HookCounters, last: bool) {
    key(out, depth, "hook_counters");
    out.push_str(" {\n");
    let entries = counters.entries();
    for (i, (name, count)) in entries.iter().enumerate() {
        let entry_last = i + 1 == entries.len();
        kv_u64(out, depth + 1, name, *count, entry_last);
    }
    indent(out, depth);
    if last {
        out.push_str("}\n");
    } else {
        out.push_str("},\n");
    }
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn key(out: &mut String, depth: usize, key: &str) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\":");
}

fn kv_u64(out: &mut String, depth: usize, key: &str, value: u64, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn kv_bool(out: &mut String, depth: usize, key: &str, value: bool, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {}", if value { "true" } else { "false" });
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn kv_str(out: &mut String, depth: usize, key: &str, value: &str, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {}", json_quote(value));
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn kv_str_opt(out: &mut String, depth: usize, key: &str, value: Option<&str>, last: bool) {
    indent(out, depth);
    match value {
        Some(v) => {
            let _ = write!(out, "\"{key}\": {}", json_quote(v));
        }
        None => {
            let _ = write!(out, "\"{key}\": null");
        }
    }
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn write_string_slice(
    out: &mut String,
    depth: usize,
    field_name: &str,
    values: &[&str],
    last: bool,
) {
    key(out, depth, field_name);
    out.push_str(" [\n");
    for (idx, value) in values.iter().enumerate() {
        let entry_last = idx + 1 == values.len();
        indent(out, depth + 1);
        out.push_str(&json_quote(value));
        if entry_last {
            out.push('\n');
        } else {
            out.push_str(",\n");
        }
    }
    indent(out, depth);
    out.push(']');
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn json_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn harness_covers_every_scenario() {
        let report = run_harness();
        assert_eq!(report.scenarios.len(), SCENARIOS.len());
    }

    #[test]
    fn harness_is_byte_stable_across_runs() {
        let a = report_to_json(&run_harness());
        let b = report_to_json(&run_harness());
        assert_eq!(a, b, "harness JSON must be byte-stable across runs");
    }

    #[test]
    fn every_scenario_reaches_its_expected_outcome() {
        let report = run_harness();
        for scenario in &report.scenarios {
            assert_eq!(
                scenario.plan.save_manifest.outcome, scenario.expected_outcome,
                "{} reached {:?} but expected {:?}",
                scenario.label, scenario.plan.save_manifest.outcome, scenario.expected_outcome,
            );
        }
    }

    #[test]
    fn alias_convergence_fires_on_alias_scenarios() {
        let report = run_harness();
        let alias_labels: BTreeSet<&'static str> = [
            "case_only_difference",
            "symlink_alias",
            "hardlink_sibling",
            "unicode_normalization_variant",
        ]
        .into_iter()
        .collect();
        for scenario in &report.scenarios {
            if alias_labels.contains(scenario.label) {
                assert!(
                    scenario.counters.vfs_alias_converge >= 1,
                    "{} did not increment vfs_alias_converge",
                    scenario.label
                );
            }
        }
    }

    #[test]
    fn read_only_and_review_paths_never_fire_atomic_commit() {
        let report = run_harness();
        for scenario in &report.scenarios {
            match scenario.plan.save_manifest.outcome {
                SaveOutcome::ReadOnlyOrPolicyBlocked | SaveOutcome::ReviewRequiredBeforeSave => {
                    assert_eq!(
                        scenario.counters.vfs_save_atomic_commit, 0,
                        "{} fired atomic commit on a blocked outcome",
                        scenario.label
                    );
                    assert!(
                        scenario.counters.vfs_save_blocked >= 1,
                        "{} did not fire vfs_save_blocked",
                        scenario.label
                    );
                }
                _ => {}
            }
        }
    }

    #[test]
    fn watcher_degradation_scenario_emits_degraded_frame() {
        let report = run_harness();
        let s = report
            .scenarios
            .iter()
            .find(|s| s.label == "watcher_degradation")
            .expect("watcher_degradation scenario present");
        let degraded = s
            .plan
            .watcher_frames
            .iter()
            .any(|f| f.watcher_health.is_degraded());
        assert!(degraded, "watcher_degradation must emit a degraded frame");
    }

    #[test]
    fn remote_conditional_conflict_uses_save_conflict_not_external_change() {
        let report = run_harness();
        let s = report
            .scenarios
            .iter()
            .find(|s| s.label == "remote_conditional_conflict")
            .unwrap();
        assert_eq!(
            s.plan.save_manifest.outcome,
            SaveOutcome::SaveConflict,
            "remote conditional root must report save_conflict, not external_change_detected"
        );
    }

    #[test]
    fn aggregate_sums_match_per_scenario() {
        let report = run_harness();
        let mut committed = 0;
        let mut external = 0;
        let mut conflict = 0;
        let mut blocked = 0;
        let mut review = 0;
        let mut participant = 0;
        for s in &report.scenarios {
            match s.plan.save_manifest.outcome {
                SaveOutcome::Committed => committed += 1,
                SaveOutcome::ExternalChangeDetected => external += 1,
                SaveOutcome::SaveConflict => conflict += 1,
                SaveOutcome::ReadOnlyOrPolicyBlocked => blocked += 1,
                SaveOutcome::ReviewRequiredBeforeSave => review += 1,
                SaveOutcome::SaveParticipantFailed => participant += 1,
                _ => {}
            }
        }
        assert_eq!(report.aggregate.total_committed, committed);
        assert_eq!(report.aggregate.total_external_change_detected, external);
        assert_eq!(report.aggregate.total_save_conflict, conflict);
        assert_eq!(report.aggregate.total_read_only_or_policy_blocked, blocked);
        assert_eq!(report.aggregate.total_review_required_before_save, review);
        assert_eq!(report.aggregate.total_save_participant_failed, participant);
    }

    #[test]
    fn scenario_labels_are_unique() {
        let labels: BTreeSet<&'static str> = SCENARIOS.iter().map(|s| s.label).collect();
        assert_eq!(
            labels.len(),
            SCENARIOS.len(),
            "scenario labels must be unique"
        );
    }

    #[test]
    fn scenario_corpus_case_ids_are_unique() {
        let ids: BTreeSet<&'static str> = SCENARIOS.iter().map(|s| s.corpus_case_id).collect();
        assert_eq!(
            ids.len(),
            SCENARIOS.len(),
            "scenario corpus_case_id values must be unique"
        );
    }

    #[test]
    fn scenario_json_is_byte_stable() {
        let report_a = run_harness();
        let report_b = run_harness();
        for (a, b) in report_a.scenarios.iter().zip(report_b.scenarios.iter()) {
            assert_eq!(scenario_to_json(a), scenario_to_json(b));
        }
    }
}
