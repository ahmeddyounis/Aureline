//! Headless inspector and regenerator for the M5 adapter-confidence audit.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the confidence-preservation fixture corpus from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_confidence_labels
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_confidence_labels -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_confidence_labels -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_confidence_labels -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_confidence_labels -- ai-evidence
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_confidence_labels -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_confidence_labels -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_adapter_confidence_audit_input, seeded_adapter_confidence_audit,
    AdapterConfidenceAudit, BuildTestInteropConfidence, OverwriteDecision, OverwriteReason,
    SourceQualityChange, ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_ID,
    ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_ID, ADAPTER_CONFIDENCE_AUDIT_FIXTURE_DIR,
    ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF, ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/adapter-confidence-audit";
const STRUCTURED_OVERCLAIM_CLAIM: &str = "claim:coverage:structured";
const BLOCKED_DECISION_CLAIM: &str = "claim:test:finish:heuristic";
const HELD_SUBJECT: &str = "subject:notebook:test";

const CASES: [(&str, &str, &str); 7] = [
    (
        "baseline_stable.json",
        "none",
        "Canonical audit binds one source-class/confidence/banner label to every claimed surface and arbitrates each contested subject so weaker re-reports are blocked or enriched, never silently accepted.",
    ),
    (
        "binding_missing_blocks_stable.json",
        "binding_missing",
        "The task-center surface has no label binding, so a claimed surface would render confidence with no provenance.",
    ),
    (
        "surface_collapses_label_blocks_stable.json",
        "surface_collapses_label",
        "A surface binding stops keeping source class and confidence distinct, collapsing them into one badge.",
    ),
    (
        "surface_hides_banner_blocks_stable.json",
        "surface_hides_banner",
        "A surface binding stops showing the heuristic-fallback banner, hiding that a row came from a fallback parser.",
    ),
    (
        "claim_confidence_overclaim_blocks_stable.json",
        "claim_confidence_overclaim",
        "A structured-output claim asserts a confidence above its source ceiling, masquerading as stronger truth.",
    ),
    (
        "lower_confidence_overwrite_accepted_blocks_stable.json",
        "lower_confidence_overwrite_accepted",
        "A weaker heuristic claim that attempted to overwrite native truth is recorded as enrich-only instead of blocked, letting it silently replace stronger truth.",
    ),
    (
        "source_quality_change_mismatch_blocks_stable.json",
        "source_quality_change_mismatch",
        "A subject stores a source-quality change that disagrees with the derived arbitration, misreporting how authority moved.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let audit = seeded_adapter_confidence_audit();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &audit),
        Some("packet") => print_json(&audit),
        Some("support-export") => print_json(
            &audit.support_export(ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_ID, exported_at()),
        ),
        Some("cli-headless") => print_json(
            &audit.cli_headless_view(ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_ID, exported_at()),
        ),
        Some("ai-evidence") => print_json(
            &audit.ai_evidence_view(ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_ID, exported_at()),
        ),
        Some("compact") => {
            for line in audit.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match audit.validate() {
            findings if findings.is_empty() => println!("ok"),
            findings => {
                for finding in &findings {
                    eprintln!("error: {}", finding.finding_kind.as_str());
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            std::process::exit(2);
        }
    }
}

fn regenerate(root: &Path, audit: &AdapterConfidenceAudit) {
    write_json(
        &root.join(ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF),
        audit,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &audit.support_export(ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &audit.cli_headless_view(ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("ai_evidence.json"),
        &audit.ai_evidence_view(ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_ID, exported_at()),
    );
    let compact = audit.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = mutated_audit(mutation);
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_adapter_confidence_audit_case",
            "schema_version": 1,
            "case_name": case_name,
            "scenario": scenario,
            "mutation": mutation,
            "expect": {
                "promotion_state": mutated.promotion_state.as_str(),
                "validation_finding_count": mutated.validation_findings.len(),
                "expected_finding_kinds": mutated
                    .validation_findings
                    .iter()
                    .map(|f| f.finding_kind.as_str())
                    .collect::<Vec<_>>(),
                "surface_tokens": mutated.surface_tokens(),
                "source_kind_tokens": mutated.source_kind_tokens(),
                "source_quality_change_tokens": mutated.source_quality_change_tokens(),
                "overwrite_decision_tokens": mutated.overwrite_decision_tokens(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(
            &root
                .join(ADAPTER_CONFIDENCE_AUDIT_FIXTURE_DIR)
                .join(file_name),
            &fixture,
        );
    }
}

/// Builds the audit for one mutation.
///
/// Input-level mutations re-materialize from a mutated seed input; packet-level
/// mutations (which write dishonest *derived* fields) tamper with the
/// materialized audit and refresh its findings. The contract test mirrors this
/// verbatim so the fixtures stay bit-for-bit derivable.
fn mutated_audit(mutation: &str) -> AdapterConfidenceAudit {
    match mutation {
        "none" => {
            AdapterConfidenceAudit::materialize(current_stable_adapter_confidence_audit_input())
        }
        "binding_missing" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            input.surface_bindings.remove(0);
            AdapterConfidenceAudit::materialize(input)
        }
        "surface_collapses_label" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            input.surface_bindings[0].keeps_source_and_confidence_distinct = false;
            AdapterConfidenceAudit::materialize(input)
        }
        "surface_hides_banner" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            input.surface_bindings[0].shows_heuristic_fallback_banner = false;
            AdapterConfidenceAudit::materialize(input)
        }
        "claim_confidence_overclaim" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            for subject in &mut input.subjects {
                for claim in &mut subject.claims {
                    if claim.claim_id == STRUCTURED_OVERCLAIM_CLAIM {
                        claim.label.confidence = BuildTestInteropConfidence::High;
                    }
                }
            }
            AdapterConfidenceAudit::materialize(input)
        }
        "lower_confidence_overwrite_accepted" => {
            let mut audit = AdapterConfidenceAudit::materialize(
                current_stable_adapter_confidence_audit_input(),
            );
            for subject in &mut audit.subjects {
                for decision in &mut subject.overwrite_decisions {
                    if decision.claim_id == BLOCKED_DECISION_CLAIM {
                        decision.decision = OverwriteDecision::EnrichedContextOnly;
                        decision.reason = Some(OverwriteReason::NeverClaimedAuthority);
                    }
                }
            }
            audit.refresh_findings();
            audit
        }
        "source_quality_change_mismatch" => {
            let mut audit = AdapterConfidenceAudit::materialize(
                current_stable_adapter_confidence_audit_input(),
            );
            for subject in &mut audit.subjects {
                if subject.subject.subject_id == HELD_SUBJECT {
                    subject.source_quality_change = SourceQualityChange::UpgradedToAuthoritative;
                }
            }
            audit.refresh_findings();
            audit
        }
        other => panic!("unknown mutation {other}"),
    }
}

fn exported_at() -> &'static str {
    "2026-06-17T00:01:00Z"
}

fn print_json(value: &impl serde::Serialize) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("serialize JSON")
    );
}

fn write_json(path: &PathBuf, value: &impl serde::Serialize) {
    ensure_parent(path);
    let payload = serde_json::to_string_pretty(value).expect("serialize JSON");
    std::fs::write(path, format!("{payload}\n")).expect("write JSON");
}

fn write_text(path: &PathBuf, body: &str) {
    ensure_parent(path);
    std::fs::write(path, format!("{body}\n")).expect("write text");
}

fn ensure_parent(path: &Path) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create artifact directory");
    }
}
