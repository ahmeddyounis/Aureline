//! Deterministic claimed-stable matrix for learnability disclosure.
//!
//! Every scenario here is projected through the **live** switching and learning
//! builders — [`crate::migration_corpus::seeded_migration_scoreboard`] for the
//! switching cohort and its incumbent terms, the live migration wizard for which
//! cohort currently has guided coverage, and
//! [`crate::learning_mode::seeded_learning_mode_beta_manifest`] /
//! [`crate::learning_mode::seeded_learning_mode_beta_surface_projection`] for the
//! stable command anchors, guided-affordance lifecycle marker, and privacy
//! posture — so the disclosure records are a genuine projection of the shell's
//! own code rather than a parallel model. The corpus mints one governed
//! [`LearnabilityDisclosureRecord`] per imported source ecosystem and pins it on
//! disk under
//! `fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance/`.
//!
//! The wizard's own live source is VS Code, so the VS Code switching cohort
//! carries a why-now card grounded in a live command anchor and qualifies
//! Stable; the other three cohorts share the same stable glossary anchors,
//! cited contextual docs, focus return, and non-blocking posture but their
//! why-now card cites docs only until guided coverage lands, so they are
//! narrowed below Stable with a named reason instead of inheriting the VS Code
//! green row. Every cohort carries the learning-mode guided affordance marked
//! `Beta`, so no cohort implies stable guided coverage by adjacency.

use crate::import::CompetitorConfigClassification;
use crate::learning_mode::{
    seeded_learning_mode_beta_manifest, seeded_learning_mode_beta_surface_projection,
    LearningModeBetaManifest, LearningModeBetaSurfaceProjection, LearningTargetRef,
};
use crate::migration_corpus::{
    seeded_migration_scoreboard, EcosystemScoreboardSection, IncumbentEcosystem,
    MigrationScoreboard,
};
use crate::migration_wizard::{seeded_migration_wizard_page, MigrationWizardPage};

use super::model::{
    required_recovery_actions, AccessibilityDisclosure, ContextualDocsDisclosure, EntryRouteRecord,
    GlossaryChip, GuidedAffordanceDisclosure, GuidedAffordanceKind, LayoutMode,
    LayoutModeDisclosure, LearnabilityClaimCeiling, LearnabilityDisclosureInput,
    LearnabilityDisclosureRecord, LearnabilityPosture, LearnabilityRouteSurface,
    LearningStatePrivacyPosture, LifecycleMarker, StableClaimClass, SurfaceParity, UpstreamRefs,
    WhyNowCard,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/learnability-disclosure";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/learnability-disclosure";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-learnability-glossary-stable";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-learnability-glossary-stable";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-learnability-glossary-stable";
const GLOSSARY_COMMAND_ID: &str = "cmd:help.open_glossary";

/// One scenario in the claimed-stable learnability disclosure matrix.
#[derive(Debug, Clone)]
pub struct LearnabilityDisclosureScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Expected source ecosystem.
    pub expected_ecosystem: IncumbentEcosystem,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected count of glossary chips.
    pub expected_glossary_chip_count: usize,
    /// Expected why-now grounding posture.
    pub expected_why_now_grounded: bool,
    record: LearnabilityDisclosureRecord,
}

impl LearnabilityDisclosureScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> LearnabilityDisclosureRecord {
        self.record.clone()
    }
}

struct EcosystemScenarioMeta {
    ecosystem: IncumbentEcosystem,
    scenario_id: &'static str,
    fixture_filename: &'static str,
}

const SCENARIO_META: &[EcosystemScenarioMeta] = &[
    EcosystemScenarioMeta {
        ecosystem: IncumbentEcosystem::VsCodeCodeOss,
        scenario_id: "learnability-disclosure:vs_code_code_oss",
        fixture_filename: "vs_code_code_oss.json",
    },
    EcosystemScenarioMeta {
        ecosystem: IncumbentEcosystem::JetBrainsFamily,
        scenario_id: "learnability-disclosure:jetbrains_family",
        fixture_filename: "jetbrains_family.json",
    },
    EcosystemScenarioMeta {
        ecosystem: IncumbentEcosystem::VimNeovim,
        scenario_id: "learnability-disclosure:vim_neovim",
        fixture_filename: "vim_neovim.json",
    },
    EcosystemScenarioMeta {
        ecosystem: IncumbentEcosystem::Emacs,
        scenario_id: "learnability-disclosure:emacs",
        fixture_filename: "emacs.json",
    },
];

/// Returns the full claimed-stable learnability disclosure matrix.
pub fn learnability_disclosure_corpus() -> Vec<LearnabilityDisclosureScenario> {
    let scoreboard = seeded_migration_scoreboard();
    let manifest = seeded_learning_mode_beta_manifest();
    let surface = seeded_learning_mode_beta_surface_projection();
    let wizard = seeded_migration_wizard_page();
    let live_ecosystem = wizard_ecosystem(wizard.descriptors.source_classification);
    let command_anchors = command_anchor_pool(&surface);

    SCENARIO_META
        .iter()
        .enumerate()
        .map(|(index, meta)| {
            let section = scoreboard
                .sections
                .iter()
                .find(|section| section.ecosystem == meta.ecosystem)
                .unwrap_or_else(|| {
                    panic!("missing scoreboard section for {}", meta.ecosystem.as_str())
                });
            let record = build_record(
                meta,
                index as u32,
                section,
                &scoreboard,
                &manifest,
                &surface,
                &wizard,
                &command_anchors,
                live_ecosystem == Some(meta.ecosystem),
            );
            LearnabilityDisclosureScenario {
                scenario_id: meta.scenario_id,
                fixture_filename: meta.fixture_filename,
                expected_ecosystem: record.source_ecosystem,
                expected_claim_class: record.stable_qualification.claim_class,
                expected_qualifies_stable: record.stable_qualification.qualifies_stable,
                expected_glossary_chip_count: record.glossary_chips.len(),
                expected_why_now_grounded: record.why_now_card.is_grounded_in_truth(),
                record,
            }
        })
        .collect()
}

/// Maps the wizard's detected source family to its incumbent ecosystem.
fn wizard_ecosystem(classification: CompetitorConfigClassification) -> Option<IncumbentEcosystem> {
    match classification {
        CompetitorConfigClassification::VSCodeWorkspaceRoot => {
            Some(IncumbentEcosystem::VsCodeCodeOss)
        }
        CompetitorConfigClassification::JetBrainsIdeaRoot => {
            Some(IncumbentEcosystem::JetBrainsFamily)
        }
        CompetitorConfigClassification::UnknownConfigRoot => None,
    }
}

/// Collects the live, deduplicated command anchors from the learning surface.
fn command_anchor_pool(surface: &LearningModeBetaSurfaceProjection) -> Vec<LearningTargetRef> {
    let mut ids: Vec<String> = Vec::new();
    for row in &surface.rows {
        if let Some(command_id) = &row.command_id {
            ids.push(command_id.clone());
        }
        for target in &row.stable_target_refs {
            if target.target_kind == "command_id" {
                if let Some(command_id) = &target.command_id {
                    ids.push(command_id.clone());
                }
            }
        }
    }
    ids.sort();
    ids.dedup();
    ids.into_iter().map(target_command).collect()
}

fn target_command(command_id: String) -> LearningTargetRef {
    LearningTargetRef {
        target_kind: "command_id".to_string(),
        command_id: Some(command_id),
        file_object_id: None,
        symbol_object_id: None,
        docs_node_id: None,
        graph_node_id: None,
        surface_object_id: None,
    }
}

fn target_docs(docs_node_id: String) -> LearningTargetRef {
    LearningTargetRef {
        target_kind: "docs_node_id".to_string(),
        command_id: None,
        file_object_id: None,
        symbol_object_id: None,
        docs_node_id: Some(docs_node_id),
        graph_node_id: None,
        surface_object_id: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn build_record(
    meta: &EcosystemScenarioMeta,
    focus_order_index: u32,
    section: &EcosystemScoreboardSection,
    scoreboard: &MigrationScoreboard,
    manifest: &LearningModeBetaManifest,
    surface: &LearningModeBetaSurfaceProjection,
    wizard: &MigrationWizardPage,
    command_anchors: &[LearningTargetRef],
    live: bool,
) -> LearnabilityDisclosureRecord {
    let ecosystem = meta.ecosystem;
    let ecosystem_token = ecosystem.as_str();
    let ecosystem_label = ecosystem.display_label();

    // --- contextual docs/help -------------------------------------------------
    let mut help_node_refs: Vec<String> = section
        .rows
        .iter()
        .flat_map(|row| row.docs_help_refs.iter().cloned())
        .collect();
    help_node_refs.extend(scoreboard.docs_help_refs.iter().cloned());
    help_node_refs.sort();
    help_node_refs.dedup();
    let contextual_docs = ContextualDocsDisclosure {
        docs_browser_ref: format!("aureline://docs-browser/learnability-{ecosystem_token}"),
        help_node_refs,
        opens_in_place: true,
    };
    let first_docs_node = contextual_docs
        .help_node_refs
        .first()
        .cloned()
        .unwrap_or_else(|| section.source_ecosystem_row_ref.clone());

    // --- glossary chips -------------------------------------------------------
    let glossary_chips: Vec<GlossaryChip> = section
        .rows
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            let anchor = command_anchors
                .get(idx % command_anchors.len().max(1))
                .cloned()
                .unwrap_or_else(|| target_docs(first_docs_node.clone()));
            GlossaryChip {
                chip_id: format!(
                    "learnability-glossary-chip:{ecosystem_token}:{}",
                    row.flow_id
                ),
                incumbent_term: row.source_object_label.clone(),
                aureline_term: row.aureline_target_label.clone(),
                explanation: row.before_after_summary.clone(),
                anchor,
                docs_help_ref: row
                    .docs_help_refs
                    .first()
                    .cloned()
                    .unwrap_or_else(|| first_docs_node.clone()),
            }
        })
        .collect();

    // --- why-now card ---------------------------------------------------------
    // The live cohort's card cites a live command anchor; the others cite docs
    // only until guided coverage lands, so their card is not yet grounded in
    // command/file/symbol truth and the row is narrowed below Stable.
    let why_now_target = if live {
        command_anchors
            .first()
            .cloned()
            .unwrap_or_else(|| target_docs(first_docs_node.clone()))
    } else {
        target_docs(first_docs_node.clone())
    };
    let why_now_card = WhyNowCard {
        card_id: format!("learnability-why-now:{ecosystem_token}"),
        headline: format!("Switching from {ecosystem_label}? Here's what changed."),
        body: format!(
            "You're importing from {ecosystem_label}. This card explains, in context, how the \
             flow in front of you maps to Aureline and links the command, file, or symbol behind \
             it so you can keep working without a tutorial detour."
        ),
        cited_target: why_now_target,
        dismissible: true,
        blocks_first_useful_work: false,
    };

    // --- learnability posture -------------------------------------------------
    let posture = LearnabilityPosture {
        opt_in: true,
        blocks_first_useful_work: false,
        preserves_exact_focus_return: true,
        focus_return_anchor_ref: format!("aureline://focus-anchor/learnability-{ecosystem_token}"),
    };

    // --- guided affordance (learning mode, Beta) ------------------------------
    let guided_affordances = vec![GuidedAffordanceDisclosure {
        affordance_id: format!("learnability-guided-tour:{ecosystem_token}"),
        affordance_kind: GuidedAffordanceKind::LearningMode,
        lifecycle_marker: LifecycleMarker::Beta,
        support_boundary: format!(
            "The optional guided tour for {ecosystem_label} switchers is Beta: its steps and \
             resume behaviour may change, so the inline cards, glossary chips, and contextual \
             docs on this row are Stable independently of the tour."
        ),
        marker_visible_in_product: true,
        marker_visible_in_docs_help: true,
        marker_visible_in_support_export: true,
    }];
    let has_guided_affordance = !guided_affordances.is_empty();

    // --- privacy posture (projected from the learning manifest) ---------------
    let privacy = privacy_posture(manifest, ecosystem_token);

    // --- claim ceiling --------------------------------------------------------
    let glossary_anchors_stable = glossary_chips.iter().all(GlossaryChip::has_stable_anchor);
    let why_now_grounded = why_now_card.is_grounded_in_truth();
    let contextual_docs_cited = contextual_docs.cites_docs_nodes();
    let focus_return_preserved = posture.preserves_exact_focus_return;
    let non_blocking = posture.is_non_blocking() && !why_now_card.blocks_first_useful_work;
    let claim_ceiling = LearnabilityClaimCeiling {
        asserts_glossary_anchors_stable: glossary_anchors_stable,
        asserts_why_now_grounded: why_now_grounded,
        asserts_contextual_docs_cited: contextual_docs_cited,
        asserts_focus_return_preserved: focus_return_preserved,
        asserts_non_blocking: non_blocking,
    };

    // --- recovery routes ------------------------------------------------------
    let recovery_actions = required_recovery_actions(has_guided_affordance);
    let recovery_routes: Vec<_> = recovery_actions
        .iter()
        .map(|action| action.route())
        .collect();
    let recovery_action_ids: Vec<String> = recovery_routes
        .iter()
        .map(|route| route.action_id.clone())
        .collect();
    let action_labels: Vec<String> = recovery_routes
        .iter()
        .map(|route| route.action_label.clone())
        .collect();

    // --- surfaces -------------------------------------------------------------
    let surfaces = SurfaceParity {
        switching_row_id: format!("switching-row:{ecosystem_token}"),
        docs_help_row_id: format!("docs-help-row:{ecosystem_token}"),
        command_palette_command_id: GLOSSARY_COMMAND_ID.to_string(),
        recovery_action_ids: recovery_action_ids.clone(),
        reopen_surfaces: vec![
            "docs_help".to_string(),
            "command_palette".to_string(),
            "support_export".to_string(),
        ],
        parity_holds: true,
    };

    // --- routes ---------------------------------------------------------------
    let routes = LearnabilityRouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!(
                "aureline://learnability-route/{}/{}",
                surface.as_str(),
                ecosystem_token
            ),
            keyboard_reachable: true,
            activates_same_row: true,
        })
        .collect();

    // --- accessibility --------------------------------------------------------
    let grounding_phrase = if why_now_grounded {
        "grounded in a command anchor"
    } else {
        "grounded in contextual docs (narrowed below Stable)"
    };
    let row_narration = format!(
        "{ecosystem_label} learnability row \u{2014} opt-in why-now card {grounding_phrase}, {} \
         glossary chips citing stable anchors, contextual docs reachable in place, optional \
         guided tour (Beta); recovery: {}.",
        glossary_chips.len(),
        action_labels.join(", ")
    );
    let accessibility = AccessibilityDisclosure {
        focus_order_index,
        tab_stop_count: 1 + recovery_routes.len() as u32,
        row_narration,
        action_labels,
        layout_modes: LayoutMode::REQUIRED
            .into_iter()
            .map(|mode| LayoutModeDisclosure {
                mode,
                row_narration_available: true,
                recovery_affordances_reachable: true,
            })
            .collect(),
    };

    // --- upstream -------------------------------------------------------------
    let upstream = UpstreamRefs {
        migration_scoreboard_ref: scoreboard.scoreboard_id.clone(),
        corpus_section_ref: section.source_ecosystem_row_ref.clone(),
        learning_manifest_ref: manifest.manifest_id.clone(),
        learning_surface_projection_ref: surface.projection_id.clone(),
    };

    let qualifier = if live {
        "qualifies Stable"
    } else {
        "narrowed below Stable"
    };
    let title = format!("{ecosystem_label} switchers: why-now, glossary, and contextual docs");
    let summary = format!(
        "Learnability layer for {ecosystem_label} switchers: an opt-in why-now card, {} glossary \
         chips mapping incumbent terms to Aureline commands, and contextual docs reachable in \
         place without a tutorial funnel; the optional guided tour is Beta; {qualifier}. (wizard \
         session {})",
        glossary_chips.len(),
        wizard.wizard_session_id
    );

    let input = LearnabilityDisclosureInput {
        record_id: meta.scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        source_ecosystem: ecosystem,
        title,
        summary,
        why_now_card,
        glossary_chips,
        contextual_docs,
        posture,
        guided_affordances,
        privacy,
        claim_ceiling,
        recovery_routes,
        surfaces,
        routes,
        accessibility,
        available_without_account: true,
        available_without_managed_services: true,
        upstream,
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };

    LearnabilityDisclosureRecord::build(input)
        .unwrap_or_else(|err| panic!("{}: {err}", meta.scenario_id))
}

/// Projects the learning-state privacy posture from the live learning manifest.
fn privacy_posture(
    manifest: &LearningModeBetaManifest,
    ecosystem_token: &str,
) -> LearningStatePrivacyPosture {
    let snapshot = manifest.progress_snapshots.first();
    let local_only_default = snapshot
        .map(|snapshot| snapshot.export_posture.local_only_default)
        .unwrap_or(true);
    let user_can_reset = snapshot
        .map(|snapshot| snapshot.export_posture.user_can_reset)
        .unwrap_or(true);
    let user_can_export = snapshot
        .map(|snapshot| snapshot.export_posture.user_can_export_metadata)
        .unwrap_or(true);
    let entries = snapshot
        .map(|snapshot| snapshot.progress_entries.as_slice())
        .unwrap_or(&[]);
    let all_inspectable = entries.iter().all(|entry| entry.inspectable_by_user);
    let all_resume_owned = entries
        .iter()
        .all(|entry| entry.inspectable_by_user && !entry.resume_ref.trim().is_empty());
    let repo_visible = entries
        .iter()
        .any(|entry| entry.repo_pack_read_default || entry.collaborator_read_default);
    let telemetry_grade = entries.iter().any(|entry| entry.telemetry_read_default);

    LearningStatePrivacyPosture {
        state_store_ref: format!("aureline://learning-state-store/{ecosystem_token}"),
        dismissals_user_owned: local_only_default && all_inspectable,
        resume_entries_user_owned: local_only_default && all_resume_owned,
        learning_digest_user_owned: user_can_reset && user_can_export && all_inspectable,
        repo_visible,
        telemetry_grade,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_covers_every_ecosystem() {
        let corpus = learnability_disclosure_corpus();
        assert_eq!(corpus.len(), SCENARIO_META.len());
        let ecosystems: Vec<IncumbentEcosystem> =
            corpus.iter().map(|s| s.expected_ecosystem).collect();
        for required in IncumbentEcosystem::required_ecosystems() {
            assert!(
                ecosystems.contains(&required),
                "missing ecosystem {required:?}"
            );
        }
    }

    #[test]
    fn corpus_spans_stable_and_narrowed_claims() {
        let corpus = learnability_disclosure_corpus();
        let stable = corpus
            .iter()
            .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
            .count();
        let narrowed = corpus
            .iter()
            .filter(|s| s.expected_claim_class != StableClaimClass::Stable)
            .count();
        assert!(stable >= 1, "matrix must include a Stable row");
        assert!(
            narrowed >= 1,
            "matrix must include a row narrowed below Stable"
        );
    }

    #[test]
    fn vs_code_row_qualifies_stable_with_grounded_why_now() {
        let corpus = learnability_disclosure_corpus();
        let vs_code = corpus
            .iter()
            .find(|s| s.expected_ecosystem == IncumbentEcosystem::VsCodeCodeOss)
            .expect("VS Code row present");
        let record = vs_code.record();
        assert_eq!(
            record.stable_qualification.claim_class,
            StableClaimClass::Stable
        );
        assert!(record.stable_qualification.qualifies_stable);
        assert!(record.stable_qualification.narrowing_reasons.is_empty());
        assert!(record.why_now_card.is_grounded_in_truth());
        assert!(record.claim_ceiling.asserts_why_now_grounded);
    }

    #[test]
    fn narrowed_rows_name_a_reason_and_drop_below_cutline() {
        for scenario in learnability_disclosure_corpus() {
            let record = scenario.record();
            if record.stable_qualification.claim_class != StableClaimClass::Stable {
                assert!(
                    !record.stable_qualification.qualifies_stable,
                    "{} narrowed but still qualifies",
                    scenario.scenario_id
                );
                assert!(
                    !record
                        .stable_qualification
                        .claim_class
                        .at_or_above_cutline(),
                    "{} narrowed claim sits above the cutline",
                    scenario.scenario_id
                );
                assert!(
                    !record.stable_qualification.narrowing_reasons.is_empty(),
                    "{} narrowed without a reason",
                    scenario.scenario_id
                );
                assert!(
                    !record.claim_ceiling.asserts_why_now_grounded,
                    "{} narrowed row over-claims why-now grounding",
                    scenario.scenario_id
                );
            }
        }
    }

    #[test]
    fn every_row_carries_a_beta_guided_affordance_marker() {
        for scenario in learnability_disclosure_corpus() {
            let record = scenario.record();
            assert!(
                !record.guided_affordances.is_empty(),
                "{} dropped the guided affordance",
                scenario.scenario_id
            );
            for affordance in &record.guided_affordances {
                assert!(
                    affordance.is_below_stable(),
                    "{} guided affordance claims Stable",
                    scenario.scenario_id
                );
                assert!(
                    affordance.marker_fully_disclosed(),
                    "{} guided affordance marker not fully disclosed",
                    scenario.scenario_id
                );
            }
            assert!(
                record.honesty_marker_present,
                "{} hides the honesty marker despite a Beta affordance",
                scenario.scenario_id
            );
        }
    }

    #[test]
    fn every_row_keeps_learning_state_local_first() {
        for scenario in learnability_disclosure_corpus() {
            let record = scenario.record();
            assert!(
                record.privacy.is_local_first(),
                "{} learning state is not local-first",
                scenario.scenario_id
            );
        }
    }

    #[test]
    fn glossary_anchors_are_stable_across_matrix() {
        for scenario in learnability_disclosure_corpus() {
            let record = scenario.record();
            assert!(
                !record.glossary_chips.is_empty(),
                "{} has no glossary chips",
                scenario.scenario_id
            );
            for chip in &record.glossary_chips {
                assert!(
                    chip.has_stable_anchor(),
                    "{} chip {} lacks a stable anchor",
                    scenario.scenario_id,
                    chip.chip_id
                );
            }
        }
    }
}
