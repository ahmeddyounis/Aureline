use std::fs;
use std::path::{Path, PathBuf};

use aureline_workspace::save::{
    SaveParticipantRiskOutcomeClass, SaveParticipantRiskReview,
    SAVE_PARTICIPANT_RISK_REVIEW_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn save_participant_risk_fixtures_parse_and_remain_support_safe() {
    let fixtures = [
        "save_participant_whole_file_block.json",
        "save_participant_source_fidelity_review.json",
    ];

    for fixture in fixtures {
        let path = repo_root()
            .join("fixtures/editor/m3/large_file_and_whole_rewrite")
            .join(fixture);
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let record: SaveParticipantRiskReview = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {fixture} must parse: {err}"));

        assert_eq!(record.record_kind, SAVE_PARTICIPANT_RISK_REVIEW_RECORD_KIND);
        assert!(record.is_support_export_safe());
        assert!(
            matches!(
                record.outcome_class,
                SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeMutation
                    | SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeCommit
            ),
            "fixture {fixture} must cover a review-required save risk"
        );
        assert!(
            !record.participant_entries.is_empty(),
            "fixture {fixture} must include participant rows"
        );
    }
}
