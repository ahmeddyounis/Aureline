//! Deterministic claimed-stable matrix for migration-flow disclosure.
//!
//! Every scenario here is projected through the **live** migration builders
//! ([`crate::migration_wizard::seeded_migration_wizard_page`] and
//! [`crate::migration_corpus::seeded_migration_scoreboard`]) so the disclosure
//! records are a genuine projection of the shell's migration code rather than a
//! parallel model. The corpus then mints one governed
//! [`MigrationFlowDisclosureRecord`] per imported source ecosystem and pins it on
//! disk under
//! `fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported/`.
//!
//! The matrix covers the four incumbent ecosystems and the full
//! Exact / Translated / Partial / Shimmed / Unsupported taxonomy. The wizard's
//! own live apply session is VS Code, so the VS Code flow carries a verified
//! pre-apply rollback (Undo + Compare) and qualifies Stable; the other three
//! ecosystems project from the corpus scoreboard, reference the same checkpoint
//! but have no live per-ecosystem rollback evidence, and are therefore narrowed
//! below Stable with a named reason instead of inheriting the VS Code green row.

use crate::import::diff_review::ImportMappingClassification;
use crate::import::CompetitorConfigClassification;
use crate::migration_corpus::{
    seeded_migration_scoreboard, EcosystemScoreboardSection, IncumbentEcosystem, IncumbentFlowRow,
    MigrationScoreboard,
};
use crate::migration_wizard::{seeded_migration_wizard_page, MigrationWizardPage};

use super::model::{
    required_recovery_actions, AccessibilityDisclosure, DiffDisclosure, EntryRouteRecord,
    GapTaxonomy, LayoutMode, LayoutModeDisclosure, MigrationClaimCeiling,
    MigrationFlowDisclosureInput, MigrationFlowDisclosureRecord, MigrationRouteSurface,
    RollbackDisclosure, StableClaimClass, SurfaceParity, UnsupportedGapDisclosure, UpstreamRefs,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/migration-flow-disclosure";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/migration-flow-disclosure";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-migration-center-stable";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-migration-center-stable";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-migration-center-stable";
const REOPEN_COMMAND_ID: &str = "cmd:migration.reopen_report";

/// One scenario in the claimed-stable migration disclosure matrix.
#[derive(Debug, Clone)]
pub struct MigrationFlowDisclosureScenario {
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
    /// Expected count of Unsupported rows.
    pub expected_unsupported: u32,
    /// Expected count of Shimmed rows.
    pub expected_shimmed: u32,
    /// Expected live-rollback posture for the flow.
    pub expected_rollback_live: bool,
    record: MigrationFlowDisclosureRecord,
}

impl MigrationFlowDisclosureScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> MigrationFlowDisclosureRecord {
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
        scenario_id: "migration-flow-disclosure:vs_code_code_oss",
        fixture_filename: "vs_code_code_oss.json",
    },
    EcosystemScenarioMeta {
        ecosystem: IncumbentEcosystem::JetBrainsFamily,
        scenario_id: "migration-flow-disclosure:jetbrains_family",
        fixture_filename: "jetbrains_family.json",
    },
    EcosystemScenarioMeta {
        ecosystem: IncumbentEcosystem::VimNeovim,
        scenario_id: "migration-flow-disclosure:vim_neovim",
        fixture_filename: "vim_neovim.json",
    },
    EcosystemScenarioMeta {
        ecosystem: IncumbentEcosystem::Emacs,
        scenario_id: "migration-flow-disclosure:emacs",
        fixture_filename: "emacs.json",
    },
];

/// Returns the full claimed-stable migration disclosure matrix.
pub fn migration_flow_disclosure_corpus() -> Vec<MigrationFlowDisclosureScenario> {
    let wizard = seeded_migration_wizard_page();
    let scoreboard = seeded_migration_scoreboard();
    let live_ecosystem = wizard_ecosystem(wizard.descriptors.source_classification);

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
                &wizard,
                &scoreboard,
                live_ecosystem == Some(meta.ecosystem),
            );
            MigrationFlowDisclosureScenario {
                scenario_id: meta.scenario_id,
                fixture_filename: meta.fixture_filename,
                expected_ecosystem: record.source_ecosystem,
                expected_claim_class: record.stable_qualification.claim_class,
                expected_qualifies_stable: record.stable_qualification.qualifies_stable,
                expected_unsupported: record.taxonomy.unsupported,
                expected_shimmed: record.taxonomy.shimmed,
                expected_rollback_live: record.rollback.is_live_for_flow(),
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

fn build_record(
    meta: &EcosystemScenarioMeta,
    focus_order_index: u32,
    section: &EcosystemScoreboardSection,
    wizard: &MigrationWizardPage,
    scoreboard: &MigrationScoreboard,
    live: bool,
) -> MigrationFlowDisclosureRecord {
    let ecosystem = meta.ecosystem;
    let ecosystem_token = ecosystem.as_str();
    let ecosystem_label = ecosystem.display_label();

    // --- taxonomy --------------------------------------------------------------
    let summary = &section.classification_summary;
    let gaps = build_gaps(section);
    let taxonomy = GapTaxonomy {
        exact: summary.exact as u32,
        translated: summary.translated as u32,
        partial: summary.partial as u32,
        shimmed: summary.shimmed as u32,
        unsupported: summary.unsupported as u32,
        classifications_present: section.classifications_present.clone(),
        unsupported_gaps_visible_before_apply: true,
        gaps,
    };
    let has_gaps = !taxonomy.gaps.is_empty();

    // --- diff ------------------------------------------------------------------
    let checkpoint_ref_raw = first_checkpoint_ref(section, scoreboard);
    let diff = DiffDisclosure {
        diff_preview_ref: format!("aureline://import-diff-preview/{ecosystem_token}"),
        reviewed_before_apply: true,
        row_count: section.rows.len() as u32,
        every_row_has_before_after: section
            .rows
            .iter()
            .all(|row| !row.before_after_summary.trim().is_empty()),
        every_row_uses_one_checkpoint: section
            .rows
            .iter()
            .all(|row| row.rollback_checkpoint_ref == checkpoint_ref_raw),
    };

    // --- rollback --------------------------------------------------------------
    let undo_available = live && !wizard.undo_actions.is_empty();
    let compare_available = live && !wizard.compare_actions.is_empty();
    let rollback = RollbackDisclosure {
        checkpoint_ref: format!("aureline://rollback-checkpoint/{ecosystem_token}"),
        restore_record_ref: format!("aureline://migration-restore-record/{ecosystem_token}"),
        created_before_apply: wizard.rollback_checkpoint.created_before_apply,
        protects_every_domain: wizard.rollback_checkpoint.protects_every_domain,
        verified_for_this_flow: live,
        undo_available,
        undo_action_ref: undo_available
            .then(|| format!("aureline://migration-undo/{ecosystem_token}")),
        compare_available,
        compare_action_ref: compare_available
            .then(|| format!("aureline://migration-compare/{ecosystem_token}")),
    };
    let rollback_live = rollback.is_live_for_flow();

    // --- claim ceiling ---------------------------------------------------------
    let claim_ceiling = MigrationClaimCeiling {
        asserts_diff_reviewed_before_apply: diff.is_reviewable_before_apply(),
        asserts_rollback_available: rollback_live,
        asserts_no_unsupported_gaps: taxonomy.has_no_gaps(),
        asserts_full_fidelity_import: taxonomy.is_full_fidelity(),
    };

    // --- recovery routes -------------------------------------------------------
    let recovery_actions = required_recovery_actions(rollback_live, has_gaps);
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

    // --- surfaces --------------------------------------------------------------
    let surfaces = SurfaceParity {
        migration_center_row_id: format!("migration-center:{ecosystem_token}"),
        settings_import_history_row_id: format!("settings-import-history:{ecosystem_token}"),
        command_palette_command_id: REOPEN_COMMAND_ID.to_string(),
        recovery_action_ids: recovery_action_ids.clone(),
        reopen_surfaces: vec![
            "settings".to_string(),
            "help".to_string(),
            "support_export".to_string(),
        ],
        parity_holds: true,
    };

    // --- routes ----------------------------------------------------------------
    let routes = MigrationRouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!(
                "aureline://migration-flow-route/{}/{}",
                surface.as_str(),
                ecosystem_token
            ),
            keyboard_reachable: true,
            activates_same_flow: true,
        })
        .collect();

    // --- accessibility ---------------------------------------------------------
    let rollback_phrase = if rollback_live {
        "available with undo and compare"
    } else {
        "referenced (no live checkpoint for this flow)"
    };
    let row_narration = format!(
        "{ecosystem_label} migration flow \u{2014} diff reviewed before apply ({} rows), \
         rollback {rollback_phrase}, taxonomy exact {}/translated {}/partial {}/shimmed \
         {}/unsupported {}; recovery: {}.",
        diff.row_count,
        taxonomy.exact,
        taxonomy.translated,
        taxonomy.partial,
        taxonomy.shimmed,
        taxonomy.unsupported,
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

    // --- upstream --------------------------------------------------------------
    let upstream = UpstreamRefs {
        wizard_session_ref: wizard.wizard_session_id.clone(),
        wizard_mapping_report_ref: wizard.mapping_report.mapping_report_id.clone(),
        rollback_checkpoint_ref: scoreboard.rollback_checkpoint_ref.clone(),
        import_diff_preview_ref: wizard.import_diff_preview_ref.clone(),
        corpus_scoreboard_ref: scoreboard.scoreboard_id.clone(),
        corpus_section_ref: section.source_ecosystem_row_ref.clone(),
    };

    let qualifier = if live {
        "qualifies Stable"
    } else {
        "narrowed below Stable"
    };
    let title = format!("{ecosystem_label} import: diff, rollback, and unsupported-gap taxonomy");
    let summary_sentence = format!(
        "{ecosystem_label} imported-user flow: {} classified rows shown as a before/after diff \
         before apply, rollback {rollback_phrase}, {} unsupported and {} shimmed gaps surfaced \
         before apply; {qualifier}.",
        taxonomy.total(),
        taxonomy.unsupported,
        taxonomy.shimmed
    );

    let input = MigrationFlowDisclosureInput {
        record_id: meta.scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        migration_session_ref: format!("aureline://migration-session/{ecosystem_token}"),
        source_ecosystem: ecosystem,
        title,
        summary: summary_sentence,
        diff,
        rollback,
        taxonomy,
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

    MigrationFlowDisclosureRecord::build(input)
        .unwrap_or_else(|err| panic!("{}: {err}", meta.scenario_id))
}

fn build_gaps(section: &EcosystemScoreboardSection) -> Vec<UnsupportedGapDisclosure> {
    let mut gaps: Vec<UnsupportedGapDisclosure> = section
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.classification,
                ImportMappingClassification::Unsupported | ImportMappingClassification::Shimmed
            )
        })
        .map(gap_from_row)
        .collect();
    gaps.sort_by(|left, right| left.gap_id.cmp(&right.gap_id));
    gaps
}

fn gap_from_row(row: &IncumbentFlowRow) -> UnsupportedGapDisclosure {
    let gap_summary = row
        .caveat
        .clone()
        .unwrap_or_else(|| format!("{} requires manual review.", row.flow_label));
    UnsupportedGapDisclosure {
        gap_id: format!("migration-flow-gap:{}", row.flow_id),
        domain: row.domain,
        classification: row.classification,
        source_label: row.source_object_label.clone(),
        gap_summary,
        visible_before_apply: true,
        retained_after_apply: true,
        docs_help_refs: row.docs_help_refs.clone(),
        support_export_refs: row.support_export_refs.clone(),
    }
}

fn first_checkpoint_ref(
    section: &EcosystemScoreboardSection,
    scoreboard: &MigrationScoreboard,
) -> String {
    section
        .rows
        .first()
        .map(|row| row.rollback_checkpoint_ref.clone())
        .unwrap_or_else(|| scoreboard.rollback_checkpoint_ref.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_covers_every_ecosystem() {
        let corpus = migration_flow_disclosure_corpus();
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
        let corpus = migration_flow_disclosure_corpus();
        let stable = corpus
            .iter()
            .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
            .count();
        let narrowed = corpus
            .iter()
            .filter(|s| s.expected_claim_class != StableClaimClass::Stable)
            .count();
        assert!(stable >= 1, "matrix must include a Stable flow");
        assert!(
            narrowed >= 1,
            "matrix must include a flow narrowed below Stable"
        );
    }

    #[test]
    fn vs_code_flow_qualifies_stable_with_live_rollback() {
        let corpus = migration_flow_disclosure_corpus();
        let vs_code = corpus
            .iter()
            .find(|s| s.expected_ecosystem == IncumbentEcosystem::VsCodeCodeOss)
            .expect("VS Code flow present");
        let record = vs_code.record();
        assert_eq!(
            record.stable_qualification.claim_class,
            StableClaimClass::Stable
        );
        assert!(record.stable_qualification.qualifies_stable);
        assert!(record.stable_qualification.narrowing_reasons.is_empty());
        assert!(record.rollback.is_live_for_flow());
        assert!(record.claim_ceiling.asserts_rollback_available);
    }

    #[test]
    fn narrowed_flows_name_a_reason_and_drop_below_cutline() {
        for scenario in migration_flow_disclosure_corpus() {
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
                    !record.claim_ceiling.asserts_rollback_available,
                    "{} narrowed flow over-claims rollback",
                    scenario.scenario_id
                );
            }
        }
    }

    #[test]
    fn stable_lane_shares_upstream_truth_with_migration_center() {
        // The stable disclosure lane must not invent a parallel migration model:
        // its upstream identities are the same wizard session, mapping report,
        // rollback checkpoint, and corpus scoreboard the migration center page
        // already pivots on.
        let page = crate::migration_center::seeded_migration_center_page();
        for scenario in migration_flow_disclosure_corpus() {
            let upstream = scenario.record().upstream;
            assert_eq!(
                upstream.wizard_session_ref, page.upstream_refs.wizard_session_ref,
                "{} wizard session drifts from migration center",
                scenario.scenario_id
            );
            assert_eq!(
                upstream.wizard_mapping_report_ref, page.upstream_refs.wizard_mapping_report_ref,
                "{} mapping report drifts from migration center",
                scenario.scenario_id
            );
            assert_eq!(
                upstream.rollback_checkpoint_ref, page.upstream_refs.wizard_rollback_checkpoint_ref,
                "{} rollback checkpoint drifts from migration center",
                scenario.scenario_id
            );
            assert_eq!(
                upstream.corpus_scoreboard_ref, page.upstream_refs.corpus_scoreboard_ref,
                "{} scoreboard drifts from migration center",
                scenario.scenario_id
            );
        }
    }

    #[test]
    fn taxonomy_spans_full_classification_across_matrix() {
        let corpus = migration_flow_disclosure_corpus();
        let mut present: std::collections::BTreeSet<ImportMappingClassification> =
            std::collections::BTreeSet::new();
        for scenario in &corpus {
            for class in &scenario.record().taxonomy.classifications_present {
                present.insert(*class);
            }
        }
        for required in [
            ImportMappingClassification::Exact,
            ImportMappingClassification::Translated,
            ImportMappingClassification::Partial,
            ImportMappingClassification::Shimmed,
            ImportMappingClassification::Unsupported,
        ] {
            assert!(present.contains(&required), "missing class {required:?}");
        }
    }
}
