//! Protected skew-test harness for the generated distributed-compatibility
//! manifests.
//!
//! The harness drives the generated verdict corpus across the four named
//! families (client/helper, client/extension, schema, provider) and proves that
//! every enumerated skew combination resolves to one closed verdict --
//! `compatible` / `probe_required` / `incompatible` -- with a repair or
//! safe-continuation guidance path, and that the harness agrees with the
//! support-export projection on every verdict the support packet quotes. An
//! unhandled skew combination is a hard failure rather than a silent pass.

use std::collections::BTreeSet;

use aureline_support::distributed_compatibility::{
    current_distributed_compatibility_skew_harness_corpus,
    current_distributed_compatibility_support_export, DistributedSkewStatusClass,
    DistributedSkewVerdict, DISTRIBUTED_COMPATIBILITY_SKEW_HARNESS_CORPUS_RECORD_KIND,
    DISTRIBUTED_COMPATIBILITY_SKEW_HARNESS_CORPUS_SCHEMA_VERSION, NO_REPAIR_REQUIRED_GUIDANCE,
};

#[test]
fn corpus_validates_and_covers_required_families() {
    let corpus = current_distributed_compatibility_skew_harness_corpus().expect("corpus parses");
    let violations = corpus.validate();

    assert_eq!(violations, Vec::new());
    assert_eq!(
        corpus.schema_version,
        DISTRIBUTED_COMPATIBILITY_SKEW_HARNESS_CORPUS_SCHEMA_VERSION
    );
    assert_eq!(
        corpus.record_kind,
        DISTRIBUTED_COMPATIBILITY_SKEW_HARNESS_CORPUS_RECORD_KIND
    );

    let families = corpus
        .families
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        families,
        BTreeSet::from(["client_extension", "client_helper", "provider", "schema"])
    );

    // Every required family enumerates at least one compatible and one
    // incompatible pairing.
    for family in ["client_helper", "client_extension", "schema", "provider"] {
        let verdicts = corpus
            .cases
            .iter()
            .filter(|case| case.manifest_family == family)
            .filter_map(|case| case.resolve_verdict())
            .collect::<BTreeSet<_>>();
        assert!(
            verdicts.contains(&DistributedSkewVerdict::Compatible),
            "{family} is missing a compatible pairing"
        );
        assert!(
            verdicts.contains(&DistributedSkewVerdict::Incompatible),
            "{family} is missing an incompatible pairing"
        );
    }
}

#[test]
fn every_case_resolves_to_a_verdict_with_a_guidance_path() {
    let corpus = current_distributed_compatibility_skew_harness_corpus().expect("corpus parses");

    for case in &corpus.cases {
        let resolved = case
            .resolve_verdict()
            .unwrap_or_else(|| panic!("case {} has an unhandled skew status", case.case_id));
        assert_eq!(
            DistributedSkewVerdict::from_token(&case.verdict),
            Some(resolved),
            "case {} declared verdict disagrees with the closed map",
            case.case_id
        );
        assert!(
            !case.guidance_path.is_empty(),
            "case {} has no guidance path",
            case.case_id
        );
        assert_eq!(
            case.mutation_blocked,
            resolved.blocks_mutation(),
            "case {} mutation flag disagrees with its verdict",
            case.case_id
        );
        if resolved == DistributedSkewVerdict::Compatible {
            assert_eq!(
                case.guidance_path,
                vec![NO_REPAIR_REQUIRED_GUIDANCE.to_owned()],
                "compatible case {} must carry the no_repair_required guidance",
                case.case_id
            );
        }
    }
}

#[test]
fn harness_agrees_with_support_export_on_every_quoted_verdict() {
    let corpus = current_distributed_compatibility_skew_harness_corpus().expect("corpus parses");
    let export = current_distributed_compatibility_support_export().expect("support export parses");

    let violations = corpus.cross_check_support_export(&export);
    assert_eq!(violations, Vec::new());

    // Each support row's current and unsupported skew cases are represented in
    // the harness with the verdict the support packet implies.
    let cases_by_ref = corpus
        .cases
        .iter()
        .map(|case| (case.skew_case_ref.as_str(), case))
        .collect::<std::collections::BTreeMap<_, _>>();
    for row in &export.support_rows {
        let current = cases_by_ref
            .get(row.current_skew_case_ref.as_str())
            .expect("current skew case present in harness");
        assert_eq!(
            current.resolve_verdict(),
            Some(DistributedSkewVerdict::Compatible)
        );
        for unsupported_ref in &row.unsupported_case_refs {
            let case = cases_by_ref
                .get(unsupported_ref.as_str())
                .expect("unsupported skew case present in harness");
            assert_eq!(
                case.resolve_verdict(),
                Some(DistributedSkewVerdict::Incompatible)
            );
            assert_eq!(case.guidance_path, row.repair_hints);
        }
    }
}

#[test]
fn status_to_verdict_map_is_total_and_closed() {
    assert_eq!(
        DistributedSkewStatusClass::Supported.verdict(),
        DistributedSkewVerdict::Compatible
    );
    assert_eq!(
        DistributedSkewStatusClass::BestEffort.verdict(),
        DistributedSkewVerdict::ProbeRequired
    );
    assert_eq!(
        DistributedSkewStatusClass::Untested.verdict(),
        DistributedSkewVerdict::ProbeRequired
    );
    assert_eq!(
        DistributedSkewStatusClass::Unsupported.verdict(),
        DistributedSkewVerdict::Incompatible
    );

    // Tokens round-trip and unknown tokens are rejected.
    for status in DistributedSkewStatusClass::ALL {
        assert_eq!(
            DistributedSkewStatusClass::from_token(status.as_str()),
            Some(status)
        );
    }
    for verdict in DistributedSkewVerdict::ALL {
        assert_eq!(
            DistributedSkewVerdict::from_token(verdict.as_str()),
            Some(verdict)
        );
    }
    assert_eq!(
        DistributedSkewStatusClass::from_token("quantum_superposed"),
        None
    );
    assert_eq!(DistributedSkewVerdict::from_token("maybe"), None);
}

#[test]
fn an_unhandled_skew_combination_fails() {
    let mut corpus =
        current_distributed_compatibility_skew_harness_corpus().expect("corpus parses");
    assert_eq!(corpus.validate(), Vec::new());

    // Introduce a skew status that the closed map does not handle.
    corpus.cases[0].status = "quantum_superposed".to_owned();
    assert_eq!(corpus.cases[0].resolve_verdict(), None);

    let check_ids = corpus
        .validate()
        .into_iter()
        .map(|violation| violation.check_id)
        .collect::<BTreeSet<_>>();
    assert!(
        check_ids.contains("case.unhandled_skew_combination"),
        "unhandled skew combination must be reported: {check_ids:?}"
    );
}

#[test]
fn dropping_a_support_export_case_breaks_the_cross_check() {
    let mut corpus =
        current_distributed_compatibility_skew_harness_corpus().expect("corpus parses");
    let export = current_distributed_compatibility_support_export().expect("support export parses");
    assert_eq!(corpus.cross_check_support_export(&export), Vec::new());

    // Drop the first case the support export quotes.
    let quoted: BTreeSet<String> = export
        .support_rows
        .iter()
        .flat_map(|row| {
            std::iter::once(row.current_skew_case_ref.clone())
                .chain(row.unsupported_case_refs.iter().cloned())
        })
        .collect();
    let index = corpus
        .cases
        .iter()
        .position(|case| quoted.contains(&case.skew_case_ref))
        .expect("a support-export-quoted case exists");
    corpus.cases.remove(index);

    let check_ids = corpus
        .cross_check_support_export(&export)
        .into_iter()
        .map(|violation| violation.check_id)
        .collect::<BTreeSet<_>>();
    assert!(
        check_ids.contains("support_export.current_case_unmatched")
            || check_ids.contains("support_export.unsupported_case_unmatched"),
        "dropping a quoted case must break the cross-check: {check_ids:?}"
    );
}
